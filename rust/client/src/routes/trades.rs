use bpx_api_types::trade::Trade;

use super::BpxClient;
use crate::error::Result;

impl BpxClient {
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<i16>) -> Result<Vec<Trade>> {
        let endpoint = format!("{}/api/v1/trades", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("symbol", symbol.to_string()),
                (
                    "limit",
                    limit.map(|l| l.to_string()).unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Trade>> {
        let endpoint = format!("{}/api/v1/trades/history", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("symbol", symbol.to_string()),
                (
                    "limit",
                    limit.map(|l| l.to_string()).unwrap_or("".to_string()),
                ),
                (
                    "offset",
                    offset.map(|o| o.to_string()).unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }
}
