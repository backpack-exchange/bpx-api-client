use bpx_api_client::{
    BACKPACK_API_BASE_URL, BpxClient,
    types::order::{ExecuteOrderPayload, OrderType, Side},
};
use clap::Parser;
use rust_decimal::prelude::*;
use std::env;

/// A simple command-line tool to execute two orders using the Backpack API batch endpoint
/// The first order will execute as a post only limit order
/// The second order will execute as a standard limit order
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The market symbol to execute orders
    #[arg(short, long, default_value = "SOL_USDC_PERP")]
    symbol: String,
    /// The side of the orders (Buy or Sell)
    #[arg(long)]
    side: Side,
    /// The price for the limit orders
    #[arg(short, long)]
    price: f32,
    // The quantity for the limit orders
    #[arg(short, long)]
    quantity: f32,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::builder()
        .base_url(base_url)
        .secret(&secret)
        .build()
        .expect("Failed to initialize Backpack API client");

    let quantity = Decimal::from_f32(args.quantity)
        .ok_or("Invalid quantity")
        .unwrap();
    let price = Decimal::from_f32(args.price)
        .ok_or("Invalid price")
        .unwrap();

    let orders = vec![
        ExecuteOrderPayload {
            symbol: args.symbol.clone(),
            side: args.side,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            post_only: Some(true),
            ..Default::default()
        },
        ExecuteOrderPayload {
            symbol: args.symbol.clone(),
            side: args.side,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            ..Default::default()
        },
    ];

    match client.execute_orders(orders).await {
        Ok(orders) => {
            let orders_json = serde_json::to_string_pretty(&orders).unwrap();
            println!("{orders_json}");
        }
        Err(err) => {
            println!("Error: {err:?}");
        }
    }
}
