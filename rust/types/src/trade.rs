use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: i64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub quote_quantity: Decimal,
    pub timestamp: i64,
    pub is_buyer_maker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Price
    #[serde(rename = "p")]
    pub price: Decimal,

    /// Quantity
    #[serde(rename = "q")]
    pub quantity: Decimal,

    /// Buyer order ID
    #[serde(rename = "b")]
    pub buyer_order_id: String,

    /// Seller order ID
    #[serde(rename = "a")]
    pub seller_order_id: String,

    /// Trade ID
    #[serde(rename = "t")]
    pub trade_id: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: i64,

    /// Is the buyer the maker?
    #[serde(rename = "m")]
    pub buyer_is_maker: bool,
}
