use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::order::{OrderStatus, Side, SystemOrderType};

#[derive(
    Debug, Display, Clone, Copy, Serialize, Deserialize, Default, EnumString, PartialEq, Eq, Hash,
)]
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
pub struct RequestForQuoteCancelPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestForQuoteRefreshPayload {
    pub rfq_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteAcceptPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    pub quote_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotePayload {
    pub rfq_id: String,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_repay: Option<bool>,
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
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
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
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
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
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
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
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
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
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// Unique RFQ order ID assigned by the matching engine.
    pub rfq_id: String,

    /// Unique RFQ quote ID, assigned by the matching engine.
    pub quote_id: String,

    /// Custom RFQ quote ID assigned by the maker (optionally)
    pub client_id: Option<u32>,

    /// Quote Bid Price.
    pub bid_price: Decimal,

    /// Quote Ask Price.
    pub ask_price: Decimal,

    /// Status.
    pub status: OrderStatus,

    /// Time the quote was created.
    pub created_at: i64,
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
    #[serde(default)]
    pub system_order_type: Option<SystemOrderType>,
}

impl QuotePayload {
    pub fn new(rfq_id: String, bid_price: Decimal, ask_price: Decimal) -> Self {
        Self {
            rfq_id,
            bid_price,
            ask_price,
            client_id: None,
            auto_lend: None,
            auto_lend_redeem: None,
            auto_borrow: None,
            auto_borrow_repay: None,
        }
    }

    pub fn with_client_id(mut self, client_id: u32) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn with_auto_lend(mut self, auto_lend: bool) -> Self {
        self.auto_lend = Some(auto_lend);
        self
    }

    pub fn with_auto_lend_redeem(mut self, auto_lend_redeem: bool) -> Self {
        self.auto_lend_redeem = Some(auto_lend_redeem);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfq_active_without_system_order_type() {
        let data = r#"{"e":"rfqActive","E":1234567890,"R":123,"s":"BTC_USDC","q":"1.5","w":1234567890,"W":1234567899,"X":"New","T":1234567890}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(data).unwrap();
        match update {
            RequestForQuoteUpdate::RfqActive {
                system_order_type, ..
            } => {
                assert!(system_order_type.is_none());
            }
            _ => panic!("Expected RfqActive"),
        }
    }

    #[test]
    fn rfq_active_with_system_order_type() {
        let data = r#"{"e":"rfqActive","E":1234567890,"R":123,"s":"BTC_USDC","q":"1.5","w":1234567890,"W":1234567899,"X":"New","T":1234567890,"o":"CollateralConversion"}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(data).unwrap();
        match update {
            RequestForQuoteUpdate::RfqActive {
                system_order_type, ..
            } => {
                assert_eq!(
                    system_order_type,
                    Some(SystemOrderType::CollateralConversion)
                );
            }
            _ => panic!("Expected RfqActive"),
        }
    }
}
