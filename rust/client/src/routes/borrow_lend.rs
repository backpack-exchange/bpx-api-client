use bpx_api_types::borrow_lend::BorrowLendPosition;

use crate::{BpxClient, Result};

#[doc(hidden)]
pub const API_BORROW_LEND_POSITIONS: &str = "/api/v1/borrowLend/positions";

impl BpxClient {
    /// Retrieves all the open borrow lending positions for the account.
    pub async fn get_borrow_lend_positions(&self) -> Result<Vec<BorrowLendPosition>> {
        let url = format!("{}{}", self.base_url, API_BORROW_LEND_POSITIONS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
