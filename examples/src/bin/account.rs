use bpx_api_client::{
    BACKPACK_API_BASE_URL, BpxClient,
    types::{account::MaxOrderQuery, order::Side},
};
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

    let params = MaxOrderQuery::new("BTC_USDC_PERP", Side::Bid);

    match client.get_account_max_order(params).await {
        Ok(max_order) => {
            let max_order_json = serde_json::to_string_pretty(&max_order).unwrap();
            println!("{max_order_json}");
        }
        Err(err) => {
            println!("Error: {err:?}");
        }
    }
}
