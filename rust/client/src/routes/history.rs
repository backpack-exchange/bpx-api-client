use bpx_api_types::fill::{Fill, FillsHistoryParams};

use crate::error::{Error, Result};
use crate::BpxClient;

#[doc(hidden)]
pub const API_FILLS_HISTORY: &str = "/wapi/v1/history/fills";

impl BpxClient {
    /// Fetches historical trades for a given symbol, with optional limit and offset.
    pub async fn get_historical_fills(&self, params: FillsHistoryParams) -> Result<Vec<Fill>> {
        let query_string =
            serde_qs::to_string(&params).map_err(|e| Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let url = format!("{}{}?{}", self.base_url, API_FILLS_HISTORY, query_string);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
