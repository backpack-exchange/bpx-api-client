use bpx_api_types::trade::Trade;

use crate::error::Result;
use crate::BpxClient;

impl BpxClient {
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<i16>) -> Result<Vec<Trade>> {
        let mut url = format!("{}/api/v1/trades?symbol={}", self.base_url, symbol);
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={}", limit));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
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
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
