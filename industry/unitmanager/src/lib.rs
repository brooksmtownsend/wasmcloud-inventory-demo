use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::*;

mod inventory;
use inventory::*;
mod ui;
use ui::Asset;

const TOPIC_PREFIX: &str = "unit.";
const UNIT_INFO: &str = "unitinfo";
const INVENTORY_KINDS: &str = "inventorykinds";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer, MessageSubscriber)]
struct UnitmanagerActor {}

#[async_trait]
impl MessageSubscriber for UnitmanagerActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let topic = msg.subject.as_str().trim_start_matches(TOPIC_PREFIX);
        let kv = KeyValueSender::new();

        match topic {
            // Listen on unit.rundown, publish all inventory contents to unit.rundown.<unit>
            "rundown" => {
                let all_inventories = all_inventories(&kv, ctx).await;
                let this_unit = kv
                    .get(ctx, UNIT_INFO)
                    .await
                    .map(|i| {
                        if i.exists {
                            i.value
                        } else {
                            "nohub".to_string()
                        }
                    })
                    .unwrap_or("nohub".to_string());
                MessagingSender::new()
                    .publish(
                        ctx,
                        &PubMessage {
                            subject: format!("hub.rundown.{this_unit}"),
                            reply_to: None,
                            body: serde_json::to_vec(&all_inventories).unwrap_or_default(),
                        },
                    )
                    .await?;
            }
            // Fallback for unknown messages
            _ => info!(
                "Received unknown message: {:?}",
                String::from_utf8_lossy(&msg.body)
            ),
        }

        Ok(())
    }
}

#[async_trait]
impl HttpServer for UnitmanagerActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(match req.path.trim_start_matches('/') {
            // Handle requests for the inventory on /inventory
            "inventory" => {
                let kv = KeyValueSender::new();
                let all_inventories = all_inventories(&kv, ctx).await;
                HttpResponse::json(all_inventories, 200)?
            }
            // Handle new shipments on /shipment
            "shipment" => {
                let kv = KeyValueSender::new();
                let incoming_inventory: InventoryItem = serde_json::from_slice(&req.body)
                    .map_err(|e| RpcError::Deser(e.to_string()))?;
                kv.increment(
                    ctx,
                    &IncrementRequest {
                        key: incoming_inventory.storage_key(),
                        value: incoming_inventory.quantity.abs(),
                    },
                )
                .await?;
                kv.set_add(
                    ctx,
                    &SetAddRequest {
                        set_name: INVENTORY_KINDS.to_string(),
                        value: incoming_inventory.item_type(),
                    },
                )
                .await?;
                HttpResponse::ok(format!(
                    "Added {} {} to inventory",
                    incoming_inventory.quantity,
                    incoming_inventory.item_type()
                ))
            }
            // Handle new orders on /order
            "order" => {
                // Same as shipments, but subtract instead of add
                let kv = KeyValueSender::new();
                let incoming_inventory: InventoryItem = serde_json::from_slice(&req.body)
                    .map_err(|e| RpcError::Deser(e.to_string()))?;
                let current_amt = kv
                    .get(ctx, &incoming_inventory.storage_key())
                    .await
                    .map(|i| i.value.parse::<i32>().unwrap_or(0))
                    .unwrap_or(0);
                if current_amt < incoming_inventory.quantity.abs() {
                    HttpResponse::bad_request(format!(
                        "Not enough {} in inventory",
                        incoming_inventory.item_type()
                    ))
                } else {
                    kv.increment(
                        ctx,
                        &IncrementRequest {
                            key: incoming_inventory.storage_key(),
                            value: -(incoming_inventory.quantity.abs()),
                        },
                    )
                    .await?;
                    HttpResponse::ok(format!(
                        "Removed {} {} from inventory",
                        incoming_inventory.quantity,
                        incoming_inventory.item_type()
                    ))
                }
            }
            // Handle setting the name of this unit
            "name" => {
                if req.method == "GET" {
                    let kv = KeyValueSender::new();
                    let name = kv
                        .get(ctx, UNIT_INFO)
                        .await
                        .map(|i| if i.exists { i.value } else { "".to_string() })
                        .unwrap_or("".to_string());
                    HttpResponse::ok(name)
                } else if req.method == "POST" {
                    let kv = KeyValueSender::new();
                    if let Ok(name) = String::from_utf8(req.body.clone()) {
                        kv.set(
                            ctx,
                            &SetRequest {
                                key: UNIT_INFO.to_string(),
                                value: name.to_string(),
                                expires: 0,
                            },
                        )
                        .await?;
                        HttpResponse::ok(format!("Set unit name to {}", name))
                    } else {
                        HttpResponse::bad_request("Invalid unit name")
                    }
                } else {
                    HttpResponse::bad_request("Invalid method")
                }
            }
            raw_path => handle_asset_request(raw_path),
        })
    }
}

/// Helper function to retrieve all inventory items
async fn all_inventories(kv: &KeyValueSender<WasmHost>, ctx: &Context) -> Vec<InventoryItem> {
    let all_categories = kv.set_query(ctx, INVENTORY_KINDS).await.unwrap_or_default();
    let mut all_inventories = vec![];
    for cat in all_categories {
        let inv: i32 = kv
            .get(ctx, &format!("inventory:{cat}"))
            .await
            .map(|i| i.value.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);
        all_inventories.push(InventoryItem::new(cat, inv));
    }
    all_inventories
}

/// Helper function to fetch the UI asset
fn handle_asset_request(raw_path: &str) -> HttpResponse {
    let path = raw_path.trim_start_matches('/');
    Asset::get(path)
        .map(|asset| response(Vec::from(asset.data), path))
        // Simple fallback to grab index.html pages when the path is the root of a page or subpage
        .or_else(|| {
            Asset::get(
                &format!("{}/index.html", path.trim_end_matches('/').to_owned())
                    .trim_start_matches('/'),
            )
            .map(|asset| response(Vec::from(asset.data), path))
        })
        .unwrap_or_else(|| HttpResponse::not_found())
}

fn response(body: Vec<u8>, path: &str) -> HttpResponse {
    let mut header = std::collections::HashMap::new();
    if let Some(content_type) = mime_guess::from_path(path).first() {
        header.insert("Content-Type".to_string(), vec![content_type.to_string()]);
    }
    HttpResponse {
        status_code: 200,
        header,
        body,
    }
}
