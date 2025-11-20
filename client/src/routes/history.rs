use bpx_api_types::fill::{Fill, FillsHistoryParams};

use crate::BpxClient;
use crate::error::{Error, Result};

#[doc(hidden)]
pub const API_FILLS_HISTORY: &str = "/wapi/v1/history/fills";

impl BpxClient {
    /// Fetches historical fills with optional filtering and pagination parameters.
    pub async fn get_historical_fills(&self, params: FillsHistoryParams) -> Result<Vec<Fill>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_FILLS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
