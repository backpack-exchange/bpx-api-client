use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::order::OrderStatus;

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
    #[serde(rename = "R")]
    pub rfq_id: u64,
    #[serde(rename = "T")]
    pub submission_time: i64,
    #[serde(rename = "w")]
    pub expiry_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "q")]
    pub quantity: Decimal,
    #[serde(rename = "X")]
    pub status: OrderStatus,
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "W")]
    pub received_time: i64,
    #[serde(rename = "E")]
    pub event_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub rfq_id: String,
    pub quote_id: String,
    pub client_id: Option<u32>,
    pub status: OrderStatus,
}
