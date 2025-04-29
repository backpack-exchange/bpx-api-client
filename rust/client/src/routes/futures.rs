use bpx_api_types::futures::Position;

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_FUTURES_POSITION: &str = "/api/v1/position";

impl BpxClient {
    pub async fn get_open_positions(&self) -> Result<Vec<Position>> {
        let url = format!("{}{}", self.base_url, API_FUTURES_POSITION);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
