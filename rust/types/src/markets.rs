use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Blockchain;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub first_price: Decimal,
    pub last_price: Decimal,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    pub trades: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub ask_price: String,

    #[serde(rename = "A")]
    pub ask_quantity: String,

    #[serde(rename = "b")]
    pub bid_price: String,

    #[serde(rename = "B")]
    pub bid_quantity: String,

    /// Update ID of event
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub base_symbol: String,
    pub quote_symbol: String,
    pub filters: MarketFilters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketFilters {
    price: PriceFilters,
    quantity: QuantityFilters,
    leverage: Option<LeverageFilters>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilters {
    min_price: Decimal,
    max_price: Option<Decimal>,
    tick_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityFilters {
    min_quantity: Decimal,
    max_quantity: Option<Decimal>,
    step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilters {
    min_leverage: Decimal,
    max_leverage: Decimal,
    step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub blockchain: Blockchain,
    pub deposit_enabled: bool,
    pub minimum_deposit: Decimal,
    pub withdrawal_enabled: bool,
    pub minimum_withdrawal: Decimal,
    pub maximum_withdrawal: Option<Decimal>,
    pub withdrawal_fee: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepth {
    pub asks: Vec<(Decimal, Decimal)>,
    pub bids: Vec<(Decimal, Decimal)>,
    pub last_update_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepthUpdate {
    /// Event type
    #[serde(rename = "e")]
    event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    symbol: String,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    timestamp: i64,

    /// First update ID in event
    #[serde(rename = "U")]
    first_update_id: u64,

    /// Last update ID in event
    #[serde(rename = "u")]
    last_update_id: u64,

    /// Asks
    #[serde(rename = "a")]
    asks: Vec<(Decimal, Decimal)>,

    /// Bids
    #[serde(rename = "b")]
    bids: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub start: String,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Option<Decimal>,
    pub end: Option<String>,
    pub volume: Decimal,
    pub trades: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub interval_end_timestamp: String,
    pub funding_rate: String,
}
