use crate::error::Result;
use std::collections::HashMap;

use bpx_api_types::{
    capital::{Balance, Deposit, RequestWithdrawalPayload, Withdrawal},
    Blockchain,
};

use super::BpxClient;

impl BpxClient {
    pub async fn get_balances(
        &self,
        subaccount_id: Option<u32>,
    ) -> Result<HashMap<String, Balance>> {
        let endpoint = format!("{}/api/v1/capital", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[(
                "subaccount_id",
                subaccount_id
                    .map(|id| id.to_string())
                    .unwrap_or("".to_string()),
            )],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn get_deposits(
        &self,
        subaccount_id: Option<u32>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Deposit>> {
        let endpoint = format!("{}/wapi/v1/capital/deposits", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                (
                    "subaccount_id",
                    subaccount_id
                        .map(|id| id.to_string())
                        .unwrap_or("".to_string()),
                ),
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

    pub async fn get_deposit_address(
        &self,
        blockchain: Blockchain,
        subaccount_id: Option<u32>,
    ) -> Result<String> {
        let endpoint = format!("{}/wapi/v1/capital/deposit/address", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("blockchain", blockchain.to_string()),
                (
                    "subaccount_id",
                    subaccount_id
                        .map(|id| id.to_string())
                        .unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn get_withdrawals(
        &self,
        subaccount_id: Option<u32>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Withdrawal>> {
        let endpoint = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                (
                    "subaccount_id",
                    subaccount_id
                        .map(|id| id.to_string())
                        .unwrap_or("".to_string()),
                ),
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

    pub async fn request_withdrawal(&self, payload: RequestWithdrawalPayload) -> Result<()> {
        let endpoint = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        self.post(endpoint, payload).await
    }
}
