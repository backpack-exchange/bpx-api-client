//! Example of authenticated vault operations (mint, redeem, cancel).
//!
//! Requires SECRET environment variable. These operations will fail without
//! sufficient balance and proper vault setup.

use bpx_api_client::{
    BACKPACK_API_BASE_URL, BpxClient,
    types::vault::{VaultMintRequest, VaultRedeemCancelRequest, VaultRedeemRequest},
};
use rust_decimal_macros::dec;
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

    // Example: mint vault tokens (deposit USDC, receive vault tokens)
    let mint_request = VaultMintRequest {
        vault_id: 1,
        symbol: "USDC".to_string(),
        quantity: dec!(50),
        auto_borrow: Some(false),
        auto_lend_redeem: Some(false),
    };

    match client.vault_mint(mint_request).await {
        Ok(()) => println!("Vault mint successful"),
        Err(err) => println!("Vault mint error: {err:?}"),
    }

    // // Example: request vault redeem (redeem vault tokens for USDC)
    let redeem_request = VaultRedeemRequest {
        vault_id: 1,
        vault_token_quantity: Some(dec!(20)),
    };

    match client.vault_redeem(redeem_request).await {
        Ok(()) => println!("Vault redeem request successful"),
        Err(err) => println!("Vault redeem error: {err:?}"),
    }

    // // Example: cancel pending vault redeem
    let cancel_request = VaultRedeemCancelRequest { vault_id: 1 };

    match client.vault_redeem_cancel(cancel_request).await {
        Ok(()) => println!("Vault redeem cancel successful"),
        Err(err) => println!("Vault redeem cancel error: {err:?}"),
    }
}
