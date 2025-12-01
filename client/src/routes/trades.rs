use bpx_api_types::trade::Trade;

use crate::BpxClient;
use crate::error::Result;

const API_TRADES: &str = "/api/v1/trades";
const API_TRADES_HISTORY: &str = "/api/v1/trades/history";

impl BpxClient {
    /// Fetches the most recent trades for a given symbol, with an optional limit.
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<i16>) -> Result<Vec<Trade>> {
        let mut url = self.base_url.join(API_TRADES)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", symbol);
            if let Some(limit) = limit {
                query.append_pair("limit", &limit.to_string());
            }
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Fetches historical trades for a given symbol, with optional limit and offset.
    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Trade>> {
        let mut url = self.base_url.join(API_TRADES_HISTORY)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", symbol);
            if let Some(limit) = limit {
                query.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = offset {
                query.append_pair("offset", &offset.to_string());
            }
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }
}
