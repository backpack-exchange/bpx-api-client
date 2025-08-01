use serde::Serialize;

use crate::{MarketType, SortDirection};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryQuery {
    /// Filter to the given order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Filter to the given strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    /// Filter to the given symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Maximum number to return. Default `100`, maximum `1000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Offset. Default `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Market type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<Vec<MarketType>>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

impl Default for OrderHistoryQuery {
    fn default() -> Self {
        Self {
            order_id: None,
            strategy_id: None,
            symbol: None,
            limit: None,
            offset: None,
            market_type: None,
            sort_direction: None,
        }
    }
} 