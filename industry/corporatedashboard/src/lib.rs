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
                let inventory: Vec<InventoryItem> = serde_json::from_slice(&msg.body)
                    .map_err(|e| RpcError::Deser(e.to_string()))?;
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
                let inv: Vec<InventoryItem> = serde_json::from_str(
                    &KeyValueSender::new()
                        // TODO: Not hardcode
                        .get(ctx, "inventory:branch:stanford")
                        .await?
                        .value,
                )
                .unwrap_or_default();

                HttpResponse::ok(serde_json::to_vec(&inv).unwrap_or_default())
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
    pub item_type: String,
    pub quantity: i32,
}
