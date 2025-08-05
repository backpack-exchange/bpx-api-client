use bpx_api_client::{BpxClient, BACKPACK_API_BASE_URL};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init(base_url, &secret, None).expect("Failed to initialize Backpack API client");

    match client.get_markets().await {
        Ok(markets) => {
            let markets_json = serde_json::to_string_pretty(&markets).unwrap();
            println!("{markets_json}");
        }
        Err(err) => tracing::error!("Error: {err:?}"),
    }
}
