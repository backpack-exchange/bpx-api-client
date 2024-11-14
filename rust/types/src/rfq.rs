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
    pub rfq_id: u64,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestForQuoteStream {
    pub stream: String,
    pub data: RequestForQuote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")] // Discriminates based on "e" field
pub enum RequestForQuote {
    RfqActive {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "q")]
        quantity: Decimal,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    QuoteAccepted {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "Q")]
        quote_id: u64,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    QuoteCancelled {
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "Q")]
        quote_id: u64,
        #[serde(rename = "X")]
        status: OrderStatus,
        #[serde(rename = "E")]
        event_time: i64,
    },
    RfqFilled {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "Q")]
        quote_id: u64,
        #[serde(rename = "S")]
        side: Side,
        #[serde(rename = "p")]
        price: Decimal,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub rfq_id: String,
    pub quote_id: String,
    pub client_id: Option<u32>,
    pub status: OrderStatus,
}
