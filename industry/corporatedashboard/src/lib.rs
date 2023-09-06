use std::collections::HashMap;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetAddRequest};
use wasmcloud_interface_numbergen::random_in_range;

mod ui;
use ui::Asset;

const JOKES_KEY: &str = "jokes";
const FALLBACK_JOKE: &str = "What do you call a fruit that's rough around the edges? A bad apple.";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct FruitjokesActor {}

#[async_trait]
impl HttpServer for FruitjokesActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // Handle new jokes
        if req.method == "POST" {
            if let Ok(joke) = String::from_utf8(req.body.clone()) {
                KeyValueSender::new()
                    .set_add(
                        ctx,
                        &SetAddRequest {
                            set_name: JOKES_KEY.to_string(),
                            value: joke,
                        },
                    )
                    .await?;
                return Ok(HttpResponse::ok("Thanks for the joke!"));
            } else {
                return Ok(HttpResponse::bad_request(
                    "That joke was bad, I'm not even going to store it",
                ));
            }
        }

        Ok(match req.path.trim_start_matches('/') {
            "joke" => {
                let all_jokes = KeyValueSender::new()
                    .set_query(ctx, &JOKES_KEY.to_string())
                    .await
                    .unwrap_or_default();
                let joke_num = random_in_range(0, (all_jokes.len() as u32).saturating_sub(1))
                    .await
                    .unwrap_or(0);
                let joke = all_jokes
                    .get(joke_num as usize)
                    .cloned()
                    .unwrap_or_else(|| FALLBACK_JOKE.to_string());

                HttpResponse::ok(joke)
            }
            raw_path => {
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
                            header
                                .insert("Content-Type".to_string(), vec![content_type.to_string()]);
                        }
                        HttpResponse {
                            status_code: 200,
                            header,
                            body: Vec::from(asset.data),
                        }
                    })
                    .unwrap_or_else(|| HttpResponse::not_found())
            }
        })
    }
}
