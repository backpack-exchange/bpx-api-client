use bpx_api_client::{
    BACKPACK_API_BASE_URL, BpxClient,
    types::{fill::FillsHistoryParams, history::SortDirection},
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init(base_url, &secret, None).expect("Failed to initialize Backpack API client");

    let params = FillsHistoryParams::default()
        .with_limit(10)
        .with_sort_direction(SortDirection::Desc);
    match client.get_historical_fills(params).await {
        Ok(fills) => {
            let fills_json = serde_json::to_string_pretty(&fills).unwrap();
            println!("{fills_json}");
        }
        Err(err) => tracing::error!("Error: {err:?}"),
    }
}
