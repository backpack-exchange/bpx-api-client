use crate::order::Side;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMaxOrder {
    pub max_order_quantity: Decimal,
    pub symbol: String,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
}

/// Payload for querying maximum order amount
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxOrderQuery {
    symbol: String,
    side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_borrow_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_lend_redeem: Option<bool>,
}

impl MaxOrderQuery {
    pub fn new<S: Into<String>>(symbol: S, side: Side) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            price: None,
            reduce_only: None,
            auto_borrow: None,
            auto_borrow_repay: None,
            auto_lend_redeem: None,
        }
    }

    pub fn with_price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn with_auto_borrow(mut self, auto_borrow: bool) -> Self {
        self.auto_borrow = Some(auto_borrow);
        self
    }

    pub fn with_auto_borrow_repay(mut self, auto_borrow_repay: bool) -> Self {
        self.auto_borrow_repay = Some(auto_borrow_repay);
        self
    }

    pub fn with_auto_lend_redeem(mut self, auto_lend_redeem: bool) -> Self {
        self.auto_lend_redeem = Some(auto_lend_redeem);
        self
    }
}
