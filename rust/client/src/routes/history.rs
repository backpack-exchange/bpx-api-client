use bpx_api_types::history::OrderHistoryQuery;
use bpx_api_types::order::Order;

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_ORDER_HISTORY: &str = "/wapi/v1/history/orders";

impl BpxClient {
    /// Retrieves the order history for the user. This includes orders that have
    /// been filled and are no longer on the book. It won't include orders
    /// that are on the book. For open orders, use the `get_open_order` api.
    pub async fn get_order_history(&self, query: Option<OrderHistoryQuery>) -> Result<Vec<Order>> {
        let mut url = format!("{}{}", self.base_url, API_ORDER_HISTORY);
        
        if let Some(query) = query {
            let mut params = vec![];
            
            if let Some(order_id) = &query.order_id {
                params.push(format!("orderId={}", order_id));
            }
            if let Some(strategy_id) = &query.strategy_id {
                params.push(format!("strategyId={}", strategy_id));
            }
            if let Some(symbol) = &query.symbol {
                params.push(format!("symbol={}", symbol));
            }
            if let Some(limit) = query.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(offset) = query.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(market_types) = &query.market_type {
                for market_type in market_types {
                    params.push(format!("marketType={}", market_type));
                }
            }
            if let Some(sort_direction) = &query.sort_direction {
                params.push(format!("sortDirection={}", sort_direction));
            }
            
            if !params.is_empty() {
                url.push('?');
                url.push_str(&params.join("&"));
            }
        }
        
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
} 