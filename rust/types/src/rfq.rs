use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::order::{OrderStatus, Side};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestForQuotePayload {
    pub quantity: Decimal,
    pub asset_in: String,
    pub asset_out: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePayload {
    rfq_id: u64,
    bid_price: Decimal,
    ask_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestForQuote {
    pub rfq_id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub submission_time: i64,
    pub expiry_time: i64,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub rfq_id: String,
    pub quote_id: String,
    pub client_id: Option<u32>,
    pub status: OrderStatus,
}
