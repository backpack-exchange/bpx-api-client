use bpx_api_client::{
    BACKPACK_API_BASE_URL, BpxClient,
    types::vault::{VaultHistoryInterval, VaultHistoryParams},
};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());

    // Public endpoints (get_vaults, get_vault_history) work without authentication
    let client = BpxClient::builder()
        .base_url(base_url)
        .build()
        .expect("Failed to initialize Backpack API client");

    match client.get_vaults().await {
        Ok(vaults) => {
            let vaults_json = serde_json::to_string_pretty(&vaults).unwrap();
            println!("Vaults:\n{vaults_json}");
        }
        Err(err) => println!("Error fetching vaults: {err:?}"),
    }

    let params = VaultHistoryParams {
        interval: VaultHistoryInterval::OneDay,
        vault_id: None,
    };

    match client.get_vault_history(params).await {
        Ok(history) => {
            let history_json = serde_json::to_string_pretty(&history).unwrap();
            println!("\nVault history (1d interval):\n{history_json}");
        }
        Err(err) => println!("Error fetching vault history: {err:?}"),
    }
}
