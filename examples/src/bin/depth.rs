use bpx_api_client::{BACKPACK_API_BASE_URL, BpxClient, types::markets::OrderBookDepthLimit};
use clap::Parser;
use std::env;

/// A simple command-line tool to fetch order book depth from the Backpack API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The market symbol to fetch the order book for (e.g., SOL_USDC)
    #[arg(short, long)]
    symbol: String,

    /// The number of bids and asks to retrieve
    #[arg(short, long, default_value_t = 5)]
    limit: u32,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());

    let client = BpxClient::builder()
        .base_url(base_url.clone())
        .build()
        .expect("Failed to initialize Backpack API client");

    let limit = OrderBookDepthLimit::try_from(args.limit).unwrap_or_else(|_| {
        println!("Invalid limit specified. Using default limit of 5.");
        OrderBookDepthLimit::Five
    });

    match client.get_order_book_depth(&args.symbol, Some(limit)).await {
        Ok(depth) => {
            let depth_json = serde_json::to_string_pretty(&depth).unwrap();
            println!("{depth_json}");
        }
        Err(err) => {
            println!("Error: {err:?}");
        }
    }
}
