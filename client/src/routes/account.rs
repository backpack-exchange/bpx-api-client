use crate::BpxClient;
use crate::error::{Error, Result};
use bpx_api_types::account::{
    AccountMaxBorrow, AccountMaxOrder, AccountMaxWithdrawal, AccountSettings, ConvertDustPayload,
    MaxOrderQuery, UpdateAccountPayload,
};

#[doc(hidden)]
pub const API_ACCOUNT: &str = "/api/v1/account";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_BORROW: &str = "/api/v1/account/limits/borrow";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_ORDER: &str = "/api/v1/account/limits/order";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_WITHDRAWAL: &str = "/api/v1/account/limits/withdrawal";
#[doc(hidden)]
pub const API_ACCOUNT_CONVERT_DUST: &str = "/api/v1/account/convertDust";

impl BpxClient {
    /// Fetches the account's settings.
    pub async fn get_account(&self) -> Result<AccountSettings> {
        let url = self.base_url.join(API_ACCOUNT)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the account's maximum borrow amount for a given symbol.
    pub async fn get_account_max_borrow(&self, symbol: &str) -> Result<AccountMaxBorrow> {
        let mut url = self.base_url.join(API_ACCOUNT_MAX_BORROW)?;
        url.query_pairs_mut().append_pair("symbol", symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the account's maximum order amount for a given symbol.
    pub async fn get_account_max_order(&self, params: MaxOrderQuery) -> Result<AccountMaxOrder> {
        let mut url = self.base_url.join(API_ACCOUNT_MAX_ORDER)?;
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| Error::UrlParseError(e.to_string().into_boxed_str()))?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the account's maximum withdrawal amount for a given symbol.
    pub async fn get_account_max_withdrawal(
        &self,
        symbol: &str,
        auto_borrow: Option<bool>,
        auto_lend_redeem: Option<bool>,
    ) -> Result<AccountMaxWithdrawal> {
        let mut url = self.base_url.join(API_ACCOUNT_MAX_WITHDRAWAL)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", symbol);
            if let Some(auto_borrow) = auto_borrow {
                query.append_pair("autoBorrow", &auto_borrow.to_string());
            }
            if let Some(auto_lend_redeem) = auto_lend_redeem {
                query.append_pair("autoLendRedeem", &auto_lend_redeem.to_string());
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Updates the account's settings.
    pub async fn update_account(&self, payload: UpdateAccountPayload) -> Result<()> {
        let url = self.base_url.join(API_ACCOUNT)?;
        self.patch(url, payload).await?;

        Ok(())
    }

    /// Converts a dust balance to USDC. The balance (including lend) must be less
    /// than the minimum quantity tradable on the spot order book.
    pub async fn convert_dust_balance(&self, payload: ConvertDustPayload) -> Result<()> {
        let url = self.base_url.join(API_ACCOUNT_CONVERT_DUST)?;
        self.post(url, payload).await?;

        Ok(())
    }
}
