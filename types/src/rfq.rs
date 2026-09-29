use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

use crate::order::{OrderStatus, Side, SystemOrderType};

#[derive(
    Debug,
    Display,
    Clone,
    Default,
    EnumString,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[strum(serialize_all = "PascalCase")]
#[non_exhaustive]
pub enum RfqExecutionMode {
    #[default]
    AwaitAccept,
    Immediate,
    /// A value this client version does not know. Carries the wire string.
    #[strum(default)]
    Unknown(String),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow_repay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_expiry_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenRfqsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_settlement: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqWithQuotes {
    pub rfq: RequestForQuote,
    pub quotes: Vec<Quote>,
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
///
/// An event type this client version does not know parses to
/// [`RequestForQuoteUpdate::Unknown`] rather than failing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e", rename_all = "camelCase")] // Discriminates based on "e" field
#[non_exhaustive]
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
        #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
        side: Option<Side>,
        #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
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
        #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
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
        #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
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
        #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "w")]
        submission_time: i64,
        #[serde(rename = "W")]
        expiry_time: i64,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
    },
    // Sent to quoters. Deferred settlement RFQs only.
    RfqAcceptedBinding {
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
        /// Side of the taker.
        #[serde(rename = "S", skip_serializing_if = "Option::is_none")]
        side: Option<Side>,
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
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
    },
    RfqCancelRequested {
        #[serde(rename = "E")]
        event_time: i64,
        #[serde(rename = "R")]
        rfq_id: u64,
        #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
        quote_id: Option<u64>,
        #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
        client_id: Option<u32>,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
        side: Option<Side>,
        #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,
        #[serde(rename = "Q", default, skip_serializing_if = "Option::is_none")]
        quote_quantity: Option<Decimal>,
        #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
        #[serde(rename = "w", default, skip_serializing_if = "Option::is_none")]
        submission_time: Option<i64>,
        #[serde(rename = "W", default, skip_serializing_if = "Option::is_none")]
        expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
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
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
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
        #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
        fill_quantity: Option<Decimal>,
        #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
        fill_quote_quantity: Option<Decimal>,
        #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
        order_expiry_time: Option<i64>,
        #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
        executed_quantity: Option<Decimal>,
        #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
        executed_quote_quantity: Option<Decimal>,
        #[serde(rename = "X")]
        order_status: OrderStatus,
        #[serde(rename = "T")]
        timestamp: i64,
        #[serde(rename = "o", default)]
        system_order_type: Option<SystemOrderType>,
    },
    /// An event type this client version does not know. Unlike the string enums, the raw
    /// `e` tag is not preserved: serde discards it while matching the tagged variants.
    #[serde(other)]
    Unknown,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_quantity: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_quote_quantity: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_expiry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_requested_at: Option<i64>,
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
    use crate::test_support::assert_wire;

    #[test]
    fn rfq_execution_mode_wire() {
        assert_wire(&RfqExecutionMode::AwaitAccept, "AwaitAccept");
        assert_wire(
            &RfqExecutionMode::Unknown("SomeFutureMode".into()),
            "SomeFutureMode",
        );
    }

    #[test]
    fn rfq_update_unknown_event_type() {
        let data = r#"{"e":"someFutureEvent","E":1234567890,"R":123,"s":"BTC_USDC","q":"1.5","w":1234567890,"W":1234567899,"X":"New","T":1234567890}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(data).unwrap();
        assert!(matches!(update, RequestForQuoteUpdate::Unknown));
    }

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

    #[test]
    fn rfq_active_resting_broadcast_carries_terms() {
        // Resting RFQs broadcast side, limit price and order expiry.
        let data = r#"{"e":"rfqActive","E":1,"R":123,"s":"BTC_USDC","S":"Bid","q":"1.5","p":"65000","w":10,"W":20,"O":90000,"z":"0","X":"New","T":1}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(data).unwrap();
        match update {
            RequestForQuoteUpdate::RfqActive {
                side,
                price,
                order_expiry_time,
                executed_quantity,
                ..
            } => {
                assert_eq!(side, Some(Side::Bid));
                assert_eq!(price, Some(Decimal::from(65000)));
                assert_eq!(order_expiry_time, Some(90000));
                assert_eq!(executed_quantity, Some(Decimal::ZERO));
            }
            _ => panic!("Expected RfqActive"),
        }
    }

    #[test]
    fn rfq_cancel_requested_requester_and_quoter_shapes() {
        // Requester copy: side and windows, no quote id.
        let taker = r#"{"e":"rfqCancelRequested","E":1,"R":123,"s":"BTC_USDC","S":"Bid","q":"1.5","p":"65000","w":10,"W":20,"O":90000,"z":"0.5","X":"PartiallyFilled","T":1}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(taker).unwrap();
        match update {
            RequestForQuoteUpdate::RfqCancelRequested {
                quote_id,
                side,
                order_expiry_time,
                order_status,
                ..
            } => {
                assert_eq!(quote_id, None);
                assert_eq!(side, Some(Side::Bid));
                assert_eq!(order_expiry_time, Some(90000));
                assert_eq!(order_status, OrderStatus::PartiallyFilled);
            }
            _ => panic!("Expected RfqCancelRequested"),
        }

        // Quoter copy: quote id, no windows or order expiry.
        let maker = r#"{"e":"rfqCancelRequested","E":1,"R":123,"u":456,"s":"BTC_USDC","S":"Ask","q":"1.5","p":"65000","X":"PartiallyFilled","T":1}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(maker).unwrap();
        match update {
            RequestForQuoteUpdate::RfqCancelRequested {
                quote_id,
                submission_time,
                expiry_time,
                order_expiry_time,
                ..
            } => {
                assert_eq!(quote_id, Some(456));
                assert_eq!(submission_time, None);
                assert_eq!(expiry_time, None);
                assert_eq!(order_expiry_time, None);
            }
            _ => panic!("Expected RfqCancelRequested"),
        }
    }

    #[test]
    fn rfq_filled_partial_slice_and_totals() {
        let data = r#"{"e":"rfqFilled","E":1,"R":123,"u":456,"s":"BTC_USDC","S":"Bid","q":"1.5","p":"64990","l":"0.5","z":"1.0","O":90000,"X":"PartiallyFilled","T":1}"#;
        let update: RequestForQuoteUpdate = serde_json::from_str(data).unwrap();
        match update {
            RequestForQuoteUpdate::RfqFilled {
                fill_quantity,
                fill_quote_quantity,
                executed_quantity,
                executed_quote_quantity,
                order_status,
                ..
            } => {
                assert_eq!(fill_quantity, Some(Decimal::new(5, 1)));
                assert_eq!(fill_quote_quantity, None);
                assert_eq!(executed_quantity, Some(Decimal::new(10, 1)));
                assert_eq!(executed_quote_quantity, None);
                assert_eq!(order_status, OrderStatus::PartiallyFilled);
            }
            _ => panic!("Expected RfqFilled"),
        }
    }

    #[test]
    fn request_for_quote_response_resting_fields() {
        let data = r#"{"rfqId":"123","symbol":"BTC_USDC","side":"Bid","price":"65000","quantity":"1.5","submissionTime":10,"expiryTime":20,"status":"PartiallyFilled","executionMode":"Immediate","createdAt":5,"executedQuantity":"0.5","orderExpiryTime":90000,"cancelRequestedAt":50000}"#;
        let rfq: RequestForQuote = serde_json::from_str(data).unwrap();
        assert_eq!(rfq.executed_quantity, Some(Decimal::new(5, 1)));
        assert_eq!(rfq.executed_quote_quantity, None);
        assert_eq!(rfq.order_expiry_time, Some(90000));
        assert_eq!(rfq.cancel_requested_at, Some(50000));

        // Non-resting responses omit these fields entirely.
        let data = r#"{"rfqId":"123","symbol":"BTC_USDC","side":"Bid","submissionTime":10,"expiryTime":20,"status":"New","executionMode":"AwaitAccept","createdAt":5}"#;
        let rfq: RequestForQuote = serde_json::from_str(data).unwrap();
        assert_eq!(rfq.order_expiry_time, None);
        assert_eq!(rfq.cancel_requested_at, None);
    }

    #[test]
    fn request_for_quote_payload_resting_serialization() {
        let payload = RequestForQuotePayload {
            client_id: None,
            quantity: Some(Decimal::new(15, 1)),
            quote_quantity: None,
            price: Some(Decimal::from(65000)),
            symbol: "BTC_USDC".into(),
            side: Side::Bid,
            execution_mode: Some(RfqExecutionMode::Immediate),
            auto_lend: None,
            auto_lend_redeem: Some(true),
            auto_borrow: Some(true),
            auto_borrow_repay: None,
            order_expiry_time: Some(90000),
        };
        let json = serde_json::to_value(&payload).unwrap();
        assert_eq!(json["orderExpiryTime"], 90000);
        assert_eq!(json["autoBorrow"], true);
        assert_eq!(json["autoLendRedeem"], true);
        assert!(json.get("autoLend").is_none());
        assert!(json.get("autoBorrowRepay").is_none());
    }

    #[test]
    fn rfq_with_quotes_deserializes() {
        let data = r#"[{"rfq":{"rfqId":"123","symbol":"BTC_USDC","side":"Bid","submissionTime":10,"expiryTime":20,"status":"New","executionMode":"AwaitAccept","createdAt":5},"quotes":[{"rfqId":"123","quoteId":"456","bidPrice":"64990","askPrice":"65010","status":"New","createdAt":6}]}]"#;
        let open: Vec<RfqWithQuotes> = serde_json::from_str(data).unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].quotes[0].quote_id, "456");
    }
}
