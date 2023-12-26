use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Blockchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub available: Decimal,
    pub locked: Decimal,
    pub staked: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub id: i32,
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub confirmation_block_number: Option<i32>,
    pub identifier: Option<String>,
    pub source: DepositSource,
    pub status: DepositStatus,
    pub subaccount_id: Option<i32>,
    pub symbol: String,
    pub quantity: Decimal,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DepositSource {
    Administrator,
    Solana,
    Ethereum,
    Bitcoin,
    Nuvei,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DepositStatus {
    Pending,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawalPayload {
    pub address: String,
    pub blockchain: Blockchain,
    pub client_id: Option<String>,
    pub quantity: Decimal,
    pub symbol: String,
    pub two_factor_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub id: i32,
    pub blockchain: Blockchain,
    pub client_id: Option<String>,
    pub identifier: Option<String>,
    pub quantity: Decimal,
    pub fee: Decimal,
    pub symbol: String,
    pub status: WithdrawalStatus,
    pub subaccount_id: Option<i32>,
    pub to_address: String,
    pub transaction_hash: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WithdrawalStatus {
    Pending,
    Confirmed,
    Verifying,
    Void,
}
