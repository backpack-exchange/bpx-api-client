use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VaultRedeemStatus {
    Requested,
    Redeemed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultRedeem {
    pub status: VaultRedeemStatus,
    pub id: String,
    pub vault_id: u32,
    pub vault_token_quantity: Decimal,
    pub vault_token: Option<String>,
    pub symbol: Option<String>,
    pub quantity: Option<Decimal>,
    pub nav: Option<Decimal>,
    pub reason: Option<String>,
    pub timestamp: i64,
}
