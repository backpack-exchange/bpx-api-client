use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSettings {
    pub auto_borrow_settlements: bool,
    pub auto_lend: bool,
    pub auto_realize_pnl: bool,
    pub auto_repay_borrows: bool,
    pub borrow_limit: Decimal,
    pub futures_maker_fee: Decimal,
    pub futures_taker_fee: Decimal,
    pub leverage_limit: Decimal,
    pub limit_orders: u32,
    pub liquidating: bool,
    pub position_limit: Decimal,
    pub spot_maker_fee: Decimal,
    pub spot_taker_fee: Decimal,
    pub trigger_orders: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMaxBorrow {
    pub max_borrow_quantity: Decimal,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMaxWithdrawal {
    pub auto_borrow: Option<bool>,
    pub auto_lend_redeem: Option<bool>,
    pub max_withdrawal_quantity: Decimal,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_settlements: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay_borrows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage_limit: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConvertDustPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}
