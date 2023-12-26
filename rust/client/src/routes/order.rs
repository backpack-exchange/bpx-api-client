use bpx_api_types::order::{ExecuteOrderPayload, Order};

use super::BpxClient;
use crate::error::Result;

impl BpxClient {
    pub async fn get_open_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<u32>,
    ) -> Result<Order> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("symbol", symbol.to_string()),
                (
                    "order_id",
                    order_id.map(|id| id.to_string()).unwrap_or("".to_string()),
                ),
                (
                    "client_id",
                    client_id.map(|id| id.to_string()).unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn execute_order(&self, payload: ExecuteOrderPayload) -> Result<Order> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        self.post(endpoint, payload).await
    }

    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        client_id: Option<&str>,
    ) -> Result<Order> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("symbol", symbol.to_string()),
                (
                    "order_id",
                    order_id.map(|id| id.to_string()).unwrap_or("".to_string()),
                ),
                (
                    "client_id",
                    client_id.map(|id| id.to_string()).unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.delete(url).await
    }

    pub async fn get_open_orders(
        &self,
        symbol: Option<&str>,
        subaccount_id: Option<u32>,
    ) -> Result<Vec<Order>> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                (
                    "symbol",
                    symbol.map(|s| s.to_string()).unwrap_or("".to_string()),
                ),
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

    pub async fn cancel_open_orders(&self, symbol: &str) -> Result<Vec<Order>> {
        let endpoint = format!("{}/api/v1/order", self.base_url);
        let url = reqwest::Url::parse_with_params(&endpoint, &[("symbol", symbol)])
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.delete(url).await
    }
}
