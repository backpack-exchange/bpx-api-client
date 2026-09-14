//! Probe: subscribe to one public stream and print the first messages.
//! With `--features permessage-deflate` the handshake offers compression;
//! run it to confirm the server selects the extension and frames decode.
use bpx_api_client::BpxClient;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ws_url =
        std::env::var("WS_URL").unwrap_or_else(|_| "wss://ws.backpack.exchange".to_string());
    // A throwaway key: public streams do not use the signature.
    let secret = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
    let client = BpxClient::builder()
        .base_url("https://api.backpack.exchange".to_string())
        .ws_url(ws_url)
        .secret(secret)
        .build()?;
    let (tx, mut rx) = mpsc::channel::<serde_json::Value>(16);
    tokio::spawn(async move {
        if let Err(e) = client.subscribe("bookTicker.SOL_USDC", tx).await {
            eprintln!("subscribe error: {e}");
        }
    });
    for _ in 0..3 {
        let msg = tokio::time::timeout(std::time::Duration::from_secs(15), rx.recv()).await?;
        println!(
            "got: {}",
            serde_json::to_string(&msg)?
                .chars()
                .take(120)
                .collect::<String>()
        );
    }
    Ok(())
}
