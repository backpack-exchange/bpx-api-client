use anyhow::Result;
use bpx_api_client::{BpxClient, BACKPACK_API_BASE_URL, BACKPACK_WS_URL};
use bpx_api_types::rfq::RequestForQuoteUpdate;
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let ws_url = env::var("WS_URL").unwrap_or_else(|_| BACKPACK_WS_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init_with_ws(base_url, ws_url, &secret, None)?;

    let (tx, mut rx) = mpsc::channel::<RequestForQuoteUpdate>(100);
    tokio::spawn(async move {
        while let Some(rfq) = rx.recv().await {
            println!("Received RFQ: {:?}", rfq);
        }
    });

    client.subscribe_to_rfqs(tx).await;

    Ok(())
}
