use bpx_api_types::borrow_lend::{
    ApyRates, BorrowLendHistory, BorrowLendMarket, BorrowLendMarketHistoryParams,
    BorrowLendPosition,
};

use crate::{BpxClient, Result};

const API_BORROW_LEND_MARKETS: &str = "/api/v1/borrowLend/markets";
const API_BORROW_LEND_MARKETS_HISTORY: &str = "/api/v1/borrowLend/markets/history";
const API_BORROW_LEND_APY: &str = "/api/v1/borrowLend/apy";

#[doc(hidden)]
pub const API_BORROW_LEND_POSITIONS: &str = "/api/v1/borrowLend/positions";

impl BpxClient {
    /// Retrieves current state of all borrow/lend markets.
    pub async fn get_borrow_lend_markets(&self) -> Result<Vec<BorrowLendMarket>> {
        let url = self.base_url.join(API_BORROW_LEND_MARKETS)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves historical snapshots of borrow/lend markets.
    pub async fn get_borrow_lend_markets_history(
        &self,
        params: BorrowLendMarketHistoryParams,
    ) -> Result<Vec<BorrowLendHistory>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_BORROW_LEND_MARKETS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves borrow/lend and staking APY rates.
    ///
    /// When `tier_id` is provided and is 6 or higher, the USDC staking yield
    /// reflects the corresponding VIP tier rate.
    pub async fn get_apy_rates(&self, tier_id: Option<i32>) -> Result<ApyRates> {
        let mut url = self.base_url.join(API_BORROW_LEND_APY)?;
        if let Some(tier_id) = tier_id {
            url.query_pairs_mut()
                .append_pair("tierId", &tier_id.to_string());
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves all the open borrow lending positions for the account.
    pub async fn get_borrow_lend_positions(&self) -> Result<Vec<BorrowLendPosition>> {
        let url = self.base_url.join(API_BORROW_LEND_POSITIONS)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
