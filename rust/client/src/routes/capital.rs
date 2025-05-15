use crate::error::Result;
use std::collections::HashMap;

use bpx_api_types::{
    capital::{Balance, Collateral, Deposit, DepositAddress, RequestWithdrawalPayload, Withdrawal},
    Blockchain,
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
        let url = format!("{}{}", self.base_url, API_CAPITAL);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves a list of deposits with optional pagination.
    pub async fn get_deposits(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Deposit>> {
        let mut url = format!("{}{}", self.base_url, API_DEPOSITS);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the deposit address for a specified blockchain.
    pub async fn get_deposit_address(&self, blockchain: Blockchain) -> Result<DepositAddress> {
        let url = format!("{}{}?blockchain={}", self.base_url, API_DEPOSIT_ADDRESS, blockchain);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves a list of withdrawals with optional pagination.
    pub async fn get_withdrawals(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Withdrawal>> {
        let mut url = format!("{}{}", self.base_url, API_WITHDRAWALS);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("{}={}&", k, v));
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Submits a withdrawal request for the specified payload.
    pub async fn request_withdrawal(&self, payload: RequestWithdrawalPayload) -> Result<Withdrawal> {
        let endpoint = format!("{}{}", self.base_url, API_WITHDRAWALS);
        let res = self.post(endpoint, payload).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the subaccount's collateral information.
    pub async fn get_collateral(&self) -> Result<Collateral> {
        let url = format!("{}{}", self.base_url, API_COLLATERAL);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
