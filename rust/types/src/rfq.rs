use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::order::{OrderStatus, Side};

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum RfqExecutionMode {
    #[default]
    AwaitAccept,
    Immediate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestForQuotePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    pub symbol: String,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<RfqExecutionMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePayload {
    pub rfq_id: String,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestForQuoteStream {
    pub stream: String,
    pub data: RequestForQuoteUpdate,
}

/// RequestForQuote updates received from the websocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")] // Discriminates based on "e" field
pub enum RequestForQuoteUpdate {
    RfqActive {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    RfqRefreshed {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S")]
        side: Side,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    RfqAccepted {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S")]
        side: Side,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    RfqCancelled {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S")]
        side: Side,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
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
        #[serde(rename = "u")]
        quote_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    QuoteCancelled {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "u")]
        quote_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    RfqCandidate {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "u")]
        quote_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S", skip_serializing_if = "Option::is_none")]
        side: Option<Side>,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "p")]
        price: Decimal,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
    },
    RfqFilled {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "u")]
        quote_id: u64,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S")]
        side: Side,
        #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestForQuote {
    pub rfq_id: String,
    pub client_id: Option<u32>,
    pub symbol: String,
    pub side: Side,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub quote_quantity: Option<Decimal>,
    pub submission_time: i64,
    pub expiry_time: i64,
    pub status: OrderStatus,
    pub execution_mode: RfqExecutionMode,
    pub created_at: i64,
}
