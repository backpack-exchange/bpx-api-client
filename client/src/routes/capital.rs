use crate::error::Result;
use std::collections::HashMap;

use bpx_api_types::{
    Blockchain,
    capital::{Balance, Collateral, Deposit, DepositAddress, RequestWithdrawalPayload, Withdrawal},
};

use crate::BpxClient;

#[doc(hidden)]
pub const API_CAPITAL: &str = "/api/v1/capital";
#[doc(hidden)]
pub const API_DEPOSITS: &str = "/wapi/v1/capital/deposits";
#[doc(hidden)]
pub const API_DEPOSIT_ADDRESS: &str = "/wapi/v1/capital/deposit/address";
#[doc(hidden)]
pub const API_WITHDRAWALS: &str = "/wapi/v1/capital/withdrawals";
#[doc(hidden)]
pub const API_COLLATERAL: &str = "/api/v1/capital/collateral";

impl BpxClient {
    /// Fetches the account's current balances.
    pub async fn get_balances(&self) -> Result<HashMap<String, Balance>> {
        let url = self.base_url.join(API_CAPITAL)?;
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Retrieves a list of deposits with optional pagination.
    pub async fn get_deposits(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Deposit>> {
        let mut url = self.base_url.join(API_DEPOSITS)?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(limit) = limit {
                query.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = offset {
                query.append_pair("offset", &offset.to_string());
            }
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Fetches the deposit address for a specified blockchain.
    pub async fn get_deposit_address(&self, blockchain: Blockchain) -> Result<DepositAddress> {
        let mut url = self.base_url.join(API_DEPOSIT_ADDRESS)?;
        url.query_pairs_mut()
            .append_pair("blockchain", &blockchain.to_string());
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Retrieves a list of withdrawals with optional pagination.
    pub async fn get_withdrawals(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Withdrawal>> {
        let mut url = self.base_url.join(API_WITHDRAWALS)?;
        {
            let mut query = url.query_pairs_mut();
            if let Some(limit) = limit {
                query.append_pair("limit", &limit.to_string());
            }
            if let Some(offset) = offset {
                query.append_pair("offset", &offset.to_string());
            }
        }
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }

    /// Submits a withdrawal request for the specified payload.
    pub async fn request_withdrawal(
        &self,
        payload: RequestWithdrawalPayload,
    ) -> Result<Withdrawal> {
        let endpoint = self.base_url.join(API_WITHDRAWALS)?;
        let res = self.post(endpoint, payload).await?;
        Self::json_with_context(res).await
    }

    /// Fetches the subaccount's collateral information.
    pub async fn get_collateral(&self) -> Result<Collateral> {
        let url = self.base_url.join(API_COLLATERAL)?;
        let res = self.get(url).await?;
        Self::json_with_context(res).await
    }
}
