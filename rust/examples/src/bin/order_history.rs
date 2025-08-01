use bpx_api_client::{BpxClient, OrderHistoryQuery, BACKPACK_API_BASE_URL};
use bpx_api_types::SortDirection;
use std::env;

#[tokio::main]
async fn main() {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init(base_url, &secret, None).expect("Failed to initialize Backpack API client");

    println!("=== CALLING GET_OPEN_ORDERS ===");
    match client.get_open_orders(Some("SOL_USDC")).await {
        Ok(orders) => println!("Open Orders parsed successfully: {} orders, {:?}", orders.len(), orders),
        Err(err) => eprintln!("Open Orders Error: {:?}", err),
    }

    println!("\n=== CALLING GET_ORDER_HISTORY ===");
    let query = OrderHistoryQuery {
        symbol: Some("SOL_USDC".to_string()),
        limit: Some(5),
        sort_direction: Some(SortDirection::Desc),
        ..Default::default()
    };

    match client.get_order_history(Some(query)).await {
        Ok(orders) => println!("Order History parsed successfully: {} orders, first order: {:?}", orders.len(), orders.first()),
        Err(err) => eprintln!("Order History Error: {:?}", err),
    }
} 