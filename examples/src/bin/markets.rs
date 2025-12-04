use bpx_api_client::{BACKPACK_API_BASE_URL, BpxClient, Error, GetMarketsRequest};
use bpx_api_types::markets::Market;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());

    let client = BpxClient::builder()
        .base_url(base_url.clone())
        .build()
        .expect("Failed to initialize Backpack API client");

    let spot_only = GetMarketsRequest::new()
        .with_spot_markets()
        .send(&client)
        .await;

    print_result(spot_only);

    let perp_only = GetMarketsRequest::new()
        .with_perp_markets()
        .send(&client)
        .await;

    print_result(perp_only);

    let prediction_only = GetMarketsRequest::new()
        .with_prediction_markets()
        .send(&client)
        .await;

    print_result(prediction_only);

    let spot_and_perp = GetMarketsRequest::new()
        .with_spot_markets()
        .with_perp_markets()
        .send(&client)
        .await;

    print_result(spot_and_perp);
}

fn print_result(res: Result<Vec<Market>, Error>) {
    match res {
        Ok(markets) => {
            let markets_json = serde_json::to_string_pretty(&markets).unwrap();
            println!("{markets_json}");
        }
        Err(err) => tracing::error!("Error: {err:?}"),
    }
}
