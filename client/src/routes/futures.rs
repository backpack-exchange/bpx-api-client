use bpx_api_types::futures::FuturePosition;

use crate::BpxClient;
use crate::error::Result;

#[doc(hidden)]
pub const API_FUTURES_POSITION: &str = "/api/v1/position";

impl BpxClient {
    pub async fn get_open_future_positions(&self) -> Result<Vec<FuturePosition>> {
        let url = self.base_url.join(API_FUTURES_POSITION)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
