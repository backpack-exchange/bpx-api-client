use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Public vault information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vault {
    /// Unique identifier for the vault.
    pub id: u32,
    /// The asset that represents shares in this vault.
    pub vault_token: String,
    /// The symbol used for minting and redeeming vault tokens.
    pub symbol: String,
    /// Whether the vault is currently accepting mints.
    pub mints_enabled: bool,
    /// Whether the vault is currently allowing redeems.
    pub redeems_enabled: bool,
    /// Minimum quantity required to mint vault tokens.
    pub min_mint_quantity: Decimal,
    /// Minimum vault token amount required to redeem.
    pub min_redeem_tokens: Decimal,
    /// Minimum delay (in milliseconds) between redeem request and execution.
    pub redeem_delay_ms: i64,
    /// Step size for vault token quantities.
    pub token_step_size: Decimal,
}

/// Request payload for minting vault tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultMintRequest {
    /// The vault ID to mint tokens from.
    pub vault_id: u32,
    /// The symbol of the asset to deposit.
    pub symbol: String,
    /// Amount to deposit.
    pub quantity: Decimal,
    /// Whether to allow auto borrowing when depositing into vault.
    pub auto_borrow: Option<bool>,
    /// Whether to allow auto redeem lent assets when depositing into vault.
    pub auto_lend_redeem: Option<bool>,
}

/// Request payload for redeeming vault tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultRedeemRequest {
    /// The vault ID to redeem from.
    pub vault_id: u32,
    /// Amount of vault tokens to deposit to redeem USDC.
    /// If not specified, uses all available vault tokens for the redemptions.
    pub vault_token_quantity: Option<Decimal>,
}

/// Request payload for canceling a vault redeem request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultRedeemCancelRequest {
    /// The vault ID to cancel the redeem request for.
    pub vault_id: u32,
}

/// Historical vault data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultHistory {
    /// The vault ID.
    pub vault_id: u32,
    /// Timestamp of the snapshot.
    pub timestamp: DateTime<Utc>,
    /// Net asset value per token.
    pub nav: Option<Decimal>,
    /// Total vault equity in USDC.
    pub vault_equity: Option<Decimal>,
    /// Total circulating vault tokens.
    pub token_circulating_supply: Option<Decimal>,
}

/// Time interval for vault history data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VaultHistoryInterval {
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1month")]
    OneMonth,
    #[serde(rename = "1year")]
    OneYear,
}

/// Parameters for fetching vault history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultHistoryParams {
    /// Time interval for historical data.
    pub interval: VaultHistoryInterval,
    /// Optional vault ID to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<u32>,
}

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
