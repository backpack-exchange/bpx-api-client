use bpx_api_client::{BACKPACK_API_BASE_URL, BpxClient};
use bpx_api_types::borrow_lend::BorrowLendMarketHistoryParams;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());

    let client = BpxClient::builder()
        .base_url(base_url.clone())
        .build()
        .expect("Failed to initialize Backpack API client");

    match client.get_borrow_lend_markets().await {
        Ok(markets) => {
            let json = serde_json::to_string_pretty(&markets).unwrap();
            println!("Borrow/lend markets:\n{json}");
        }
        Err(err) => tracing::error!("Error fetching borrow/lend markets: {err:?}"),
    }

    let history_params = BorrowLendMarketHistoryParams {
        interval: "1d".to_string(),
        symbol: Some("USDC".to_string()),
    };
    match client.get_borrow_lend_markets_history(history_params).await {
        Ok(history) => {
            let json = serde_json::to_string_pretty(&history).unwrap();
            println!("\nUSDC borrow/lend history (1d):\n{json}");
        }
        Err(err) => tracing::error!("Error fetching borrow/lend history: {err:?}"),
    }

    match client.get_apy_rates(None).await {
        Ok(rates) => {
            let json = serde_json::to_string_pretty(&rates).unwrap();
            println!("\nAPY rates:\n{json}");
        }
        Err(err) => tracing::error!("Error fetching APY rates: {err:?}"),
    }
}
