use bpx_api_types::trade::Trade;

use crate::error::Result;
use crate::BpxClient;

const API_TRADES: &str = "/api/v1/trades";
const API_TRADES_HISTORY: &str = "/api/v1/trades/history";

impl BpxClient {
    /// Fetches the most recent trades for a given symbol, with an optional limit.
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<i16>) -> Result<Vec<Trade>> {
        let mut url = format!("{}{}?symbol={}", self.base_url, API_TRADES, symbol);
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={limit}"));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches historical trades for a given symbol, with optional limit and offset.
    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Trade>> {
        let mut url = format!("{}{}?symbol={}", self.base_url, API_TRADES_HISTORY, symbol);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("&{k}={v}"));
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
