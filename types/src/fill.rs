use super::{history::SortDirection, order::Side};
use rust_decimal::Decimal;

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
pub enum FillType {
    User,
    BookLiquidation,
    Adl,
    Backstop,
    Liquidation,
    AllLiquidation,
    CollateralConversion,
    CollateralConversionAndSpotLiquidation,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub trade_id: Option<i64>,
    pub client_id: Option<String>,
    pub order_id: String,
    pub symbol: String,
    pub fee_symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub fee: Decimal,
    pub side: Side,
    pub timestamp: String,
    pub is_maker: bool,
    pub system_order_type: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FillsHistoryParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// From timestamp in milliseconds
    pub from: Option<i64>,
    /// To timestamp in milliseconds
    pub to: Option<i64>,
    /// Filter by fill type
    pub fill_type: Option<FillType>,
    /// Filter by market type
    pub market_type: Option<String>,
    /// Filter by order ID
    pub order_id: Option<String>,
    /// Filter by strategy ID
    pub strategy_id: Option<String>,
    /// Maximum number of results to return
    pub limit: Option<u64>,
    /// Offset for pagination - default to 0
    pub offset: Option<u64>,
    /// Sort direction
    pub sort_direction: Option<SortDirection>,
}

impl FillsHistoryParams {
    pub fn with_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn with_from(mut self, from: i64) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_to(mut self, to: i64) -> Self {
        self.to = Some(to);
        self
    }

    pub fn with_fill_type(mut self, fill_type: FillType) -> Self {
        self.fill_type = Some(fill_type);
        self
    }

    pub fn with_market_type(mut self, market_type: String) -> Self {
        self.market_type = Some(market_type);
        self
    }

    pub fn with_order_id<S: Into<String>>(mut self, order_id: S) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    pub fn with_strategy_id<S: Into<String>>(mut self, strategy_id: S) -> Self {
        self.strategy_id = Some(strategy_id.into());
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}
