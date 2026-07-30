use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{Blockchain, margin::MarginFunction};

/// An asset is most of the time a crypto coin that can have multiple representations
/// across different blockchains. For example, USDT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// CoinGecko ID for price tracking
    pub coingecko_id: Option<String>,
    /// Human-readable display name
    pub display_name: String,
    /// Identifier
    pub symbol: String,
    /// See [`Token`]
    pub tokens: Vec<Token>,
}

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

/// Sent by an exchange to indicate a change in the order book, such as the execution of a bid or ask.
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
    pub ask_price: Decimal,

    #[serde(rename = "A")]
    pub ask_quantity: Decimal,

    #[serde(rename = "b")]
    pub bid_price: Decimal,

    #[serde(rename = "B")]
    pub bid_quantity: Decimal,

    /// Update ID of event
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: u64,
}

/// A market is where two assets are exchanged. Most notably, in a `BTC/USDC` pair
/// `BTC` is the base and `USDC` is the quote.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// The `Market` identifier.
    pub symbol: String,
    /// The base asset.
    pub base_symbol: String,
    /// The quote asset for the market.
    pub quote_symbol: String,
    /// The type of the market. Can be `SPOT`, `PERP`, `IPERP`, `DATED`, `PREDICTION`, `RFQ` or `MONAD`.
    /// New market types may also be added in the future.
    pub market_type: String,
    /// The real-world-asset type backing the market, when the market is a
    /// tokenized RWA (e.g. `STOCK` for tokenized equities such as
    /// `SPCX.US_USDC`). `None` for regular crypto markets.
    #[serde(default)]
    pub rwa_market_type: Option<RwaMarketType>,
    /// See [`MarketFilters`].
    pub filters: MarketFilters,
    /// IMF function.
    pub imf_function: Option<MarginFunction>,
    /// MMF function.
    pub mmf_function: Option<MarginFunction>,
    /// Funding interval for perpetuals in milliseconds.
    pub funding_interval: Option<u64>,
    /// Funding rate upper bound for perpetual markets, in basis points (e.g. `10` = 10 bps).
    pub funding_rate_upper_bound: Option<Decimal>,
    /// Funding rate lower bound for perpetual markets, in basis points (e.g. `-10` = -10 bps).
    pub funding_rate_lower_bound: Option<Decimal>,
    /// Maximum open interest limit for the market, when applicable.
    pub open_interest_limit: Option<Decimal>,
    /// Current state of the market's order book.
    pub order_book_state: OrderBookState,
    /// Market created at time.
    pub created_at: chrono::NaiveDateTime,
    /// Whether the market is currently listed/visible on the exchange.
    pub visible: bool,
    /// Position limit weight coefficient used when clearing trades.
    pub position_limit_weight: Option<Decimal>,
}

impl Market {
    /// Returns the decimal places this market supports on the price.
    /// We error if a price with more decimal places is provided.
    /// `Price decimal too long`
    pub const fn price_decimal_places(&self) -> u32 {
        self.filters.price.tick_size.scale()
    }

    /// Returns the decimal places this market supports on the quantity.
    /// if you provide a more precise quantity you will get an error
    /// `Quantity decimal too long`
    pub const fn quantity_decimal_places(&self) -> u32 {
        self.filters.quantity.step_size.scale()
    }
}

/// The state of a market's order book.
///
/// New states may be added by the exchange in the future; unrecognized values
/// deserialize to [`OrderBookState::Unknown`].
#[derive(
    Debug,
    strum::Display,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumString,
    PartialEq,
    Eq,
    Hash,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderBookState {
    /// Normal operation: accepting and matching orders.
    Open,
    /// Not accepting any orders.
    Closed,
    /// Only cancellation of existing orders; no execution.
    CancelOnly,
    /// Only accepting limit orders.
    LimitOnly,
    /// Only accepting orders that would not immediately match.
    PostOnly,
    /// Any state not recognized by this client version.
    #[serde(other)]
    Unknown,
}

/// The type of real-world asset backing a tokenized RWA market.
///
/// New types may be added by the exchange in the future; unrecognized values
/// deserialize to [`RwaMarketType::Unknown`].
#[derive(
    Debug,
    strum::Display,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumString,
    PartialEq,
    Eq,
    Hash,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RwaMarketType {
    /// A tokenized equity.
    Stock,
    /// A tokenized index.
    Index,
    /// A tokenized commodity.
    Commodity,
    /// A tokenized foreign-exchange pair.
    Fx,
    /// Any type not recognized by this client version.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketFilters {
    pub price: PriceFilter,
    pub quantity: QuantityFilter,
    pub leverage: Option<LeverageFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum price the order book will allow.
    pub min_price: Decimal,
    /// Maximum price the order book will allow.
    pub max_price: Option<Decimal>,
    /// Price increment.
    pub tick_size: Decimal,
    /// Maximum allowed multiplier move from last active price.
    pub max_multiplier: Option<Decimal>,
    /// Minimum allowed multiplier move from last active price.
    pub min_multiplier: Option<Decimal>,
    /// Maximum allowed impact multiplier from last active price. This
    /// determines how far above the best ask a market buy can penetrate.
    pub max_impact_multiplier: Option<Decimal>,
    /// Minimum allowed impact multiplier from last active price. This
    /// determines how far below the best bid a market sell can penetrate.
    pub min_impact_multiplier: Option<Decimal>,
    /// Price band for futures markets. Restricts the price moving too far from
    /// the mean mark price.
    pub mean_mark_price_band: Option<PriceBandMarkPrice>,
    /// Price band for futures markets. Restricts the premium moving too far
    /// from the mean premium.
    pub mean_premium_band: Option<PriceBandPremium>,
    /// RWA price band restricting how far the price may move from the latest
    /// external oracle price.
    pub discovery_bound_band: Option<PriceBandDiscoveryBound>,
    /// Maximum allowed multiplier move from last active price without
    /// incurring an entry fee when borrowing for spot margin.
    pub borrow_entry_fee_max_multiplier: Option<Decimal>,
    /// Minimum allowed multiplier move from last active price without
    /// incurring an entry fee when borrowing for spot margin.
    pub borrow_entry_fee_min_multiplier: Option<Decimal>,
    /// Maximum allowed multiplier for mark/index price updates relative to the
    /// previous mark/index price.
    pub max_price_update_multiplier: Option<Decimal>,
    /// Minimum allowed multiplier for mark/index price updates relative to the
    /// previous mark/index price.
    pub min_price_update_multiplier: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBandMarkPrice {
    /// Maximum allowed multiplier move from mean price.
    pub max_multiplier: Decimal,
    /// Minimum allowed multiplier move from mean price.
    pub min_multiplier: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBandPremium {
    /// Latest index price.
    pub index_price: Option<Decimal>,
    /// Maximum premium the order book will allow. This is constantly updated
    /// based on the mean premium scaled by the `tolerance_pct`
    pub max_premium_pct: Option<Decimal>,
    /// Minimum premium the order book will allow. This is constantly updated
    /// based on the mean premium scaled by the `tolerance_pct`
    pub min_premium_pct: Option<Decimal>,
    /// Maximum allowed deviation from the mean premium. E.g. if
    /// `tolerance_pct` is 0.05 (5%), and the mean premium is 5%, then
    /// orders will be prevented from being placed if the premium exceeds 10%.
    /// User to calculate `min_premium_pct` and `max_premium_pct`.
    pub tolerance_pct: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBandDiscoveryBound {
    /// Maximum allowed multiplier move from the external oracle price.
    pub max_multiplier: Decimal,
    /// Minimum allowed multiplier move from the external oracle price.
    pub min_multiplier: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityFilter {
    pub min_quantity: Decimal,
    pub max_quantity: Option<Decimal>,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: Decimal,
    pub max_leverage: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub blockchain: Blockchain,
    pub contract_address: String,
    pub deposit_enabled: bool,
    pub display_name: String,
    pub minimum_deposit: Decimal,
    pub withdraw_enabled: bool,
    pub minimum_withdrawal: Decimal,
    pub maximum_withdrawal: Option<Decimal>,
    pub withdrawal_fee: Decimal,
}

#[derive(Debug, Serialize, Deserialize, strum::AsRefStr)]
pub enum OrderBookDepthLimit {
    #[serde(rename = "5")]
    #[strum(serialize = "5")]
    Five,
    #[serde(rename = "10")]
    #[strum(serialize = "10")]
    Ten,
    #[serde(rename = "20")]
    #[strum(serialize = "20")]
    Twenty,
    #[serde(rename = "50")]
    #[strum(serialize = "50")]
    Fifty,
    #[serde(rename = "100")]
    #[strum(serialize = "100")]
    OneHundred,
    #[serde(rename = "500")]
    #[strum(serialize = "500")]
    FiveHundred,
    #[serde(rename = "1000")]
    #[strum(serialize = "1000")]
    OneThousand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepth {
    /// Resting limit orders on ask side, listed as price-quantity pairs
    pub asks: Vec<(Decimal, Decimal)>,
    /// Resting limit orders on bid side, listed as price-quantity pairs
    pub bids: Vec<(Decimal, Decimal)>,
    /// The id of the last update applied to this order book state
    // The API currently returns i64 encoded as a string. This was likely done to work around
    // JavaScript's inability to handle large integers using the Number type.
    // We should change the API at some point to return an i64 instead of a string, but it's
    // a breaking change, so just improving the client for now.
    #[serde(deserialize_with = "deserialize_str_or_i64")]
    pub last_update_id: i64,
    /// Timestamp in microseconds
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepthUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: i64,

    /// First update ID in event
    #[serde(rename = "U")]
    pub first_update_id: i64,

    /// Last update ID in event
    #[serde(rename = "u")]
    pub last_update_id: i64,

    /// Asks
    #[serde(rename = "a")]
    pub asks: Vec<(Decimal, Decimal)>,

    /// Bids
    #[serde(rename = "b")]
    pub bids: Vec<(Decimal, Decimal)>,
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
    pub trades: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// K-Line start time in seconds
    #[serde(rename = "t")]
    pub start_time: i64,

    /// K-Line end time in seconds
    #[serde(rename = "T")]
    pub end_time: i64,

    /// Open price
    #[serde(rename = "o")]
    pub open_price: Decimal,

    /// Close price
    #[serde(rename = "c")]
    pub close_price: Decimal,

    /// High price
    #[serde(rename = "h")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "l")]
    pub low_price: Decimal,

    /// Base asset volume
    #[serde(rename = "v")]
    pub volume: Decimal,

    /// Number of trades
    #[serde(rename = "n")]
    pub trades: u64,

    /// Is this k-line closed?
    #[serde(rename = "X")]
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub interval_end_timestamp: String,
    pub funding_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    pub funding_rate: Decimal,
    pub index_price: Decimal,
    pub mark_price: Decimal,
    pub next_funding_timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Mark Price
    #[serde(rename = "p")]
    pub mark_price: Decimal,

    /// Estimated funding rate
    #[serde(rename = "f")]
    pub funding_rate: Decimal,

    /// Index Price
    #[serde(rename = "i")]
    pub index_price: Decimal,

    /// Next funding timestamp in microseconds
    #[serde(rename = "n")]
    pub funding_timestamp: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub engine_timestamp: i64,
}

/// A tradable equity security.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Security {
    /// Asset symbol.
    pub asset: String,
    /// CUSIP identifier for the security.
    pub cusip: Option<String>,
    /// Name.
    pub name: String,
    /// Trading sessions, each with their own quantity limits.
    pub sessions: Vec<SecuritySession>,
}

/// Trading session limits for a [`Security`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySession {
    /// Session name (e.g. `US_EQUITIES_REGULAR`).
    pub name: String,
    /// Minimum order quantity for this session.
    pub min_quantity: Decimal,
    /// Maximum order quantity for this session.
    pub max_quantity: Option<Decimal>,
    /// Minimum quantity increment for this session.
    pub step_size: Decimal,
}

impl TryFrom<u32> for OrderBookDepthLimit {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(OrderBookDepthLimit::Five),
            10 => Ok(OrderBookDepthLimit::Ten),
            20 => Ok(OrderBookDepthLimit::Twenty),
            50 => Ok(OrderBookDepthLimit::Fifty),
            100 => Ok(OrderBookDepthLimit::OneHundred),
            500 => Ok(OrderBookDepthLimit::FiveHundred),
            1000 => Ok(OrderBookDepthLimit::OneThousand),
            _ => Err("Invalid OrderBookDepthLimit value"),
        }
    }
}

/// Deserializes a value that can be either a string or an i64 into an i64.
fn deserialize_str_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Visitor;
    use std::fmt;

    struct StringOrI64Visitor;

    impl<'de> Visitor<'de> for StringOrI64Visitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or an integer")
        }

        fn visit_str<E>(self, value: &str) -> Result<i64, E>
        where
            E: serde::de::Error,
        {
            value.parse().map_err(serde::de::Error::custom)
        }

        fn visit_i64<E>(self, value: i64) -> Result<i64, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<i64, E>
        where
            E: serde::de::Error,
        {
            i64::try_from(value).map_err(|_| serde::de::Error::custom("value too large"))
        }
    }

    deserializer.deserialize_any(StringOrI64Visitor)
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    fn get_test_market() -> Market {
        Market {
            symbol: "TEST_MARKET".to_string(),
            base_symbol: "TEST".to_string(),
            quote_symbol: "MARKET".to_string(),
            market_type: "SPOT".to_string(),
            rwa_market_type: None,
            filters: super::MarketFilters {
                price: PriceFilter {
                    min_price: dec!(0.0001),
                    max_price: None,
                    tick_size: dec!(0.0001),
                    min_multiplier: Some(dec!(1.25)),
                    max_multiplier: Some(dec!(0.75)),
                    max_impact_multiplier: Some(dec!(1.05)),
                    min_impact_multiplier: Some(dec!(0.95)),
                    mean_mark_price_band: None,
                    mean_premium_band: None,
                    discovery_bound_band: None,
                    borrow_entry_fee_max_multiplier: None,
                    borrow_entry_fee_min_multiplier: None,
                    max_price_update_multiplier: None,
                    min_price_update_multiplier: None,
                },
                quantity: QuantityFilter {
                    min_quantity: dec!(0.01),
                    max_quantity: None,
                    step_size: dec!(0.01),
                },
                leverage: None,
            },
            imf_function: None,
            mmf_function: None,
            funding_interval: None,
            funding_rate_upper_bound: None,
            funding_rate_lower_bound: None,
            open_interest_limit: None,
            order_book_state: OrderBookState::Open,
            created_at: chrono::NaiveDateTime::parse_from_str(
                "2025-01-21T06:34:54.691858",
                "%Y-%m-%dT%H:%M:%S%.f",
            )
            .unwrap(),
            visible: true,
            position_limit_weight: None,
        }
    }

    #[test]
    fn test_decimal_places_on_price_filters_4() {
        let market = get_test_market();
        assert_eq!(market.price_decimal_places(), 4);
    }

    #[test]
    fn test_decimal_places_on_quantity_filters() {
        let market = get_test_market();
        assert_eq!(market.quantity_decimal_places(), 2);
    }

    #[test]
    fn test_mark_price_update_parse() {
        let data = r#"
{
	"E": 1747291031914525,
	"T": 1747291031910025,
	"e": "markPrice",
	"f": "-0.0000039641039274236048482914",
	"i": "173.44031179",
	"n": 1747296000000,
	"p": "173.35998175",
	"s": "SOL_USDC_PERP"
}
        "#;

        let mark_price_update: MarkPriceUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(mark_price_update.symbol, "SOL_USDC_PERP".to_string());
        assert_eq!(
            mark_price_update.funding_rate,
            dec!(-0.0000039641039274236048482914)
        );
        assert_eq!(mark_price_update.mark_price, dec!(173.35998175));
    }

    #[test]
    fn test_kline_update_parse() {
        let data = r#"
{
  "e": "kline",
  "E": 1694687692980000,
  "s": "SOL_USD",
  "t": 123400000,
  "T": 123460000,
  "o": "18.75",
  "c": "19.25",
  "h": "19.80",
  "l": "18.50",
  "v": "32123",
  "n": 93828,
  "X": false
}
        "#;

        let kline_update: KlineUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(kline_update.symbol, "SOL_USD".to_string());
        assert_eq!(kline_update.start_time, 123400000);
        assert_eq!(kline_update.open_price, dec!(18.75));
    }

    #[test]
    fn test_order_book_depth_last_update_id_as_string() {
        let data = r#"
{
  "asks": [["18.70", "0.000"]],
  "bids": [["18.67", "0.832"]],
  "lastUpdateId": "94978271",
  "timestamp": 1694687965941000
}
        "#;

        let depth: OrderBookDepth = serde_json::from_str(data).unwrap();
        assert_eq!(depth.last_update_id, 94978271);
    }

    #[test]
    fn test_order_book_depth_last_update_id_as_i64() {
        let data = r#"
{
  "asks": [["18.70", "0.000"]],
  "bids": [["18.67", "0.832"]],
  "lastUpdateId": 94978271,
  "timestamp": 1694687965941000
}
        "#;

        let depth: OrderBookDepth = serde_json::from_str(data).unwrap();
        assert_eq!(depth.last_update_id, 94978271);
    }

    #[test]
    fn test_market_order_book_state_and_visible_parse() {
        let data = r#"
{
  "symbol": "WEN_USDC",
  "baseSymbol": "WEN",
  "quoteSymbol": "USDC",
  "marketType": "SPOT",
  "filters": {
    "price": { "minPrice": "0.0001", "tickSize": "0.0001" },
    "quantity": { "minQuantity": "0.01", "stepSize": "0.01" }
  },
  "orderBookState": "Closed",
  "createdAt": "2025-01-21T06:34:54.691858",
  "visible": false
}
        "#;

        let market: Market = serde_json::from_str(data).unwrap();
        assert_eq!(market.order_book_state, OrderBookState::Closed);
        assert!(!market.visible);

        // Unrecognized states fall back to `Unknown` rather than failing the parse.
        let unknown = data.replace("\"Closed\"", "\"SomeFutureState\"");
        let market: Market = serde_json::from_str(&unknown).unwrap();
        assert_eq!(market.order_book_state, OrderBookState::Unknown);
    }

    #[test]
    fn test_market_rwa_market_type_parse() {
        let data = r#"
{
  "symbol": "SPCX.US_USDC",
  "baseSymbol": "SPCX.US",
  "quoteSymbol": "USDC",
  "marketType": "SPOT",
  "rwaMarketType": "STOCK",
  "filters": {
    "price": { "minPrice": "0.01", "tickSize": "0.01" },
    "quantity": { "minQuantity": "0.01", "stepSize": "0.01" }
  },
  "orderBookState": "PostOnly",
  "createdAt": "2025-01-21T06:34:54.691858",
  "visible": false
}
        "#;

        let market: Market = serde_json::from_str(data).unwrap();
        assert_eq!(market.rwa_market_type, Some(RwaMarketType::Stock));

        // The exchange's full RWA set: STOCK, INDEX, COMMODITY, FX.
        for (wire, expected) in [
            ("INDEX", RwaMarketType::Index),
            ("COMMODITY", RwaMarketType::Commodity),
            ("FX", RwaMarketType::Fx),
        ] {
            let payload = data.replace("STOCK", wire);
            let market: Market = serde_json::from_str(&payload).unwrap();
            assert_eq!(market.rwa_market_type, Some(expected));
        }

        // Explicit null parses to `None`, as the API sends for crypto markets.
        let null = data.replace("\"STOCK\"", "null");
        let market: Market = serde_json::from_str(&null).unwrap();
        assert_eq!(market.rwa_market_type, None);

        // An absent key parses to `None` for responses predating the field.
        let absent = data.replace("\"rwaMarketType\": \"STOCK\",", "");
        let market: Market = serde_json::from_str(&absent).unwrap();
        assert_eq!(market.rwa_market_type, None);

        // Unrecognized types fall back to `Unknown` rather than failing the parse.
        let unknown = data.replace("\"STOCK\"", "\"SOME_FUTURE_RWA\"");
        let market: Market = serde_json::from_str(&unknown).unwrap();
        assert_eq!(market.rwa_market_type, Some(RwaMarketType::Unknown));
    }

    #[test]
    fn test_market_perp_fields_parse() {
        let data = r#"
{
  "baseSymbol": "SOL",
  "createdAt": "2025-01-21T06:34:54.691858",
  "filters": {
    "price": {
      "borrowEntryFeeMaxMultiplier": null,
      "borrowEntryFeeMinMultiplier": null,
      "maxImpactMultiplier": "1.01",
      "maxMultiplier": "2",
      "maxPrice": "1000",
      "meanMarkPriceBand": {
        "maxMultiplier": "1.1",
        "minMultiplier": "0.9"
      },
      "meanPremiumBand": {
        "tolerancePct": "0.05"
      },
      "minImpactMultiplier": "0.99",
      "minMultiplier": "0.25",
      "minPrice": "0.01",
      "tickSize": "0.01"
    },
    "quantity": {
      "maxQuantity": null,
      "minQuantity": "0.01",
      "stepSize": "0.01"
    }
  },
  "fundingInterval": 3600000,
  "fundingRateLowerBound": "-100",
  "fundingRateUpperBound": "100",
  "imfFunction": {
    "base": "0.02",
    "factor": "0.00006",
    "type": "sqrt"
  },
  "marketType": "PERP",
  "mmfFunction": {
    "base": "0.0135",
    "factor": "0.000036",
    "type": "sqrt"
  },
  "openInterestLimit": "4000000",
  "orderBookState": "Open",
  "positionLimitWeight": "1",
  "quoteSymbol": "USDC",
  "symbol": "SOL_USDC_PERP",
  "visible": true
}
        "#;

        let market: Market = serde_json::from_str(data).unwrap();
        assert_eq!(market.symbol, "SOL_USDC_PERP");
        assert_eq!(market.funding_interval, Some(3600000));
        assert_eq!(market.funding_rate_lower_bound, Some(dec!(-100)));
        assert_eq!(market.funding_rate_upper_bound, Some(dec!(100)));
        assert_eq!(market.open_interest_limit, Some(dec!(4000000)));
        assert_eq!(market.position_limit_weight, Some(dec!(1)));
        assert_eq!(market.imf_function.as_ref().unwrap().function_type, "sqrt");
        assert_eq!(market.imf_function.as_ref().unwrap().base, dec!(0.02));
        assert_eq!(
            market
                .filters
                .price
                .mean_premium_band
                .as_ref()
                .unwrap()
                .tolerance_pct,
            dec!(0.05)
        );
    }
}
