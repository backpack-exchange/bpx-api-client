use bpx_api_client::{BACKPACK_API_BASE_URL, BpxClient};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::builder()
        .base_url(base_url)
        .secret(&secret)
        .build()
        .expect("Failed to initialize Backpack API client");

    match client.get_open_orders(Some("SOL_USDC")).await {
        Ok(orders) => println!("Open Orders: {orders:?}"),
        Err(err) => tracing::error!("Error: {err:?}"),
    }
}
