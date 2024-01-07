use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::*;
use wasmcloud_interface_messaging::*;

mod ui;
use ui::Asset;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer, MessageSubscriber)]
struct HubdashboardActor {}

#[async_trait]
impl MessageSubscriber for HubdashboardActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let topic = msg.subject.as_str().trim_start_matches("hub.rundown.");
        let kv = KeyValueSender::new();

        match topic {
            "" => (),
            unit => {
                // De/serialization trip just to validate typing
                let mut inventory: Vec<InventoryItem> = serde_json::from_slice(&msg.body)
                    .map_err(|e| RpcError::Deser(e.to_string()))?;
                inventory.iter_mut().for_each(|item| {
                    item.unit = unit.to_string();
                });
                kv.set(
                    ctx,
                    &SetRequest {
                        key: format!("inventory:unit:{unit}"),
                        value: serde_json::to_string(&inventory)
                            .map_err(|e| RpcError::Ser(e.to_string()))?,
                        expires: 0,
                    },
                )
                .await?;
                kv.set_add(
                    ctx,
                    &SetAddRequest {
                        set_name: "units".to_string(),
                        value: unit.to_string(),
                    },
                )
                .await?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl HttpServer for HubdashboardActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(match req.path.trim_start_matches('/') {
            "rundown" => {
                MessagingSender::new()
                    .publish(
                        ctx,
                        &PubMessage {
                            subject: "unit.rundown".to_string(),
                            reply_to: None,
                            body: serde_json::to_vec("heyjimletmegetthatrundownrealquick")
                                .unwrap_or_default(),
                        },
                    )
                    .await?;
                HttpResponse::ok("Rundown requested")
            }
            "clear" => {
                let all_units = KeyValueSender::new()
                    .set_query(ctx, "units")
                    .await
                    .unwrap_or_default();
                let kv = KeyValueSender::new();
                for unit in all_units {
                    kv.del(ctx, &format!("inventory:unit:{unit}")).await?;
                    kv.set_del(
                        ctx,
                        &SetDelRequest {
                            set_name: "units".to_string(),
                            value: unit,
                        },
                    )
                    .await?;
                }
                HttpResponse::ok("Inventory cleared")
            }
            "inventory" => {
                let all_units = KeyValueSender::new()
                    .set_query(ctx, "units")
                    .await
                    .unwrap_or_default();
                let mut all_inventories: Vec<Vec<InventoryItem>> = vec![];
                for unit in all_units {
                    let inv: Vec<InventoryItem> = serde_json::from_str(
                        &KeyValueSender::new()
                            .get(ctx, &format!("inventory:unit:{unit}"))
                            .await?
                            .value,
                    )
                    .unwrap_or_default();
                    all_inventories.push(inv);
                }

                HttpResponse::ok(serde_json::to_vec(&all_inventories).unwrap_or_default())
            }
            raw_path => handle_asset_request(raw_path),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unit: String,
    pub item_type: String,
    pub quantity: i32,
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
    let mut header = HashMap::new();
    if let Some(content_type) = mime_guess::from_path(path).first() {
        header.insert("Content-Type".to_string(), vec![content_type.to_string()]);
    }
    HttpResponse {
        status_code: 200,
        header,
        body,
    }
}
