use bpx_api_types::trade::Trade;

use super::BpxClient;
use crate::error::Result;

impl BpxClient {
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<i16>) -> Result<Vec<Trade>> {
        let mut url = format!("{}/api/v1/trades?symbol={}", self.base_url, symbol);
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }
        self.get(url).await
    }

    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Trade>> {
        let mut url = format!("{}/api/v1/trades/history?symbol={}", self.base_url, symbol);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        self.get(url).await
    }
}
