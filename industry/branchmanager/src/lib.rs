use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::*;

mod inventory;
use inventory::*;

const TOPIC_PREFIX: &str = "munderdifflin.";
const BRANCH_INFO: &str = "branchinfo";
const INVENTORY_KINDS: &str = "inventorykinds";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer, MessageSubscriber)]
struct BranchmanagerActor {}

#[async_trait]
impl MessageSubscriber for BranchmanagerActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let topic = msg.subject.as_str().trim_start_matches(TOPIC_PREFIX);
        let kv = KeyValueSender::new();

        match topic {
            // Listen on munderdifflin.rundown, publish all inventory contents to munderdifflin.rundown.<branch>
            "rundown" => {
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
                let this_branch = kv
                    .get(ctx, BRANCH_INFO)
                    .await
                    .map(|i| {
                        if i.exists {
                            i.value
                        } else {
                            "michaelscottpapercompany".to_string()
                        }
                    })
                    .unwrap_or("michaelscottpapercompany".to_string());
                MessagingSender::new()
                    .publish(
                        ctx,
                        &PubMessage {
                            subject: format!("corporate.rundown.{this_branch}"),
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
impl HttpServer for BranchmanagerActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(match req.path.trim_start_matches('/') {
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
            _ => HttpResponse::not_found(),
        })
    }
}
