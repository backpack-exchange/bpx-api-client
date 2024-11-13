use bpx_api_client::{BpxClient, BACKPACK_API_BASE_URL, BACKPACK_WS_URL};
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let ws_url = env::var("WS_URL").unwrap_or_else(|_| BACKPACK_WS_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");
    let stream = env::var("STREAM").expect("Missing STREAM environment variable");

    let client = BpxClient::init_with_ws(base_url, ws_url, &secret, None)?;
    client.subscribe(&stream).await;

    Ok(())
}
