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
pub struct RequestForQuoteStream {
    pub stream: String,
    pub data: RequestForQuote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")] // Discriminates based on "e" field
pub enum RequestForQuote {
    RfqActive {
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "T")]
        submission_time: i64,
        #[serde(rename = "w")]
        expiry_time: i64,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "q")]
        quantity: Decimal,
        #[serde(rename = "X")]
        status: OrderStatus,
        #[serde(rename = "W")]
        received_time: i64,
        #[serde(rename = "E")]
        event_time: i64,
    },
    QuoteAccepted {
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "Q")]
        quote_id: u64,
        #[serde(rename = "T")]
        submission_time: i64,
        #[serde(rename = "X")]
        status: OrderStatus,
        #[serde(rename = "E")]
        event_time: i64,
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
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "Q")]
        quote_id: u64,
        #[serde(rename = "T")]
        submission_time: i64,
        #[serde(rename = "X")]
        status: OrderStatus,
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "S")]
        side: String,
        #[serde(rename = "p")]
        price: Decimal,
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
