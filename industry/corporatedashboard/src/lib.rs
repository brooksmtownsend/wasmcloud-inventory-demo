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
struct CorporatedashboardActor {}

#[async_trait]
impl MessageSubscriber for CorporatedashboardActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let topic = msg
            .subject
            .as_str()
            .trim_start_matches("corporate.rundown.");
        let kv = KeyValueSender::new();

        match topic {
            "" => (),
            branch => {
                // De/serialization trip just to validate typing
                let mut inventory: Vec<InventoryItem> = serde_json::from_slice(&msg.body)
                    .map_err(|e| RpcError::Deser(e.to_string()))?;
                inventory.iter_mut().for_each(|item| {
                    item.branch = branch.to_string();
                });
                kv.set(
                    ctx,
                    &SetRequest {
                        key: format!("inventory:branch:{branch}"),
                        value: serde_json::to_string(&inventory)
                            .map_err(|e| RpcError::Ser(e.to_string()))?,
                        expires: 0,
                    },
                )
                .await?;
                kv.set_add(
                    ctx,
                    &SetAddRequest {
                        set_name: "branches".to_string(),
                        value: branch.to_string(),
                    },
                )
                .await?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl HttpServer for CorporatedashboardActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        Ok(match req.path.trim_start_matches('/') {
            "rundown" => {
                MessagingSender::new()
                    .publish(
                        ctx,
                        &PubMessage {
                            subject: "munderdifflin.rundown".to_string(),
                            reply_to: None,
                            body: serde_json::to_vec("heyjimletmegetthatrundownrealquick")
                                .unwrap_or_default(),
                        },
                    )
                    .await?;
                HttpResponse::ok("Rundown requested")
            }
            "inventory" => {
                let all_branches = KeyValueSender::new()
                    .set_query(ctx, "branches")
                    .await
                    .unwrap_or_default();
                let mut all_inventories: Vec<Vec<InventoryItem>> = vec![];
                for branch in all_branches {
                    let inv: Vec<InventoryItem> = serde_json::from_str(
                        &KeyValueSender::new()
                            .get(ctx, &format!("inventory:branch:{branch}"))
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

/// Helper function to fetch the UI asset
fn handle_asset_request(raw_path: &str) -> HttpResponse {
    let path = if raw_path.is_empty() {
        "index.html"
    } else {
        raw_path
    };
    // Request for UI asset
    Asset::get(path)
        .map(|asset| {
            let mut header = HashMap::new();
            if let Some(content_type) = mime_guess::from_path(path).first() {
                header.insert("Content-Type".to_string(), vec![content_type.to_string()]);
            }
            HttpResponse {
                status_code: 200,
                header,
                body: Vec::from(asset.data),
            }
        })
        .unwrap_or_else(|| HttpResponse::not_found())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub branch: String,
    pub item_type: String,
    pub quantity: i32,
}
