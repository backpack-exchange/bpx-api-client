use std::collections::HashMap;

use bpx_api_types::markets::{Kline, Market, OrderBookDepth, Ticker, Token};

use super::BpxClient;
use crate::error::Result;

impl BpxClient {
    pub async fn get_assets(&self) -> Result<HashMap<String, Vec<Token>>> {
        let endpoint = format!("{}/api/v1/assets", self.base_url);
        self.get(endpoint).await
    }

    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        let endpoint = format!("{}/api/v1/markets", self.base_url);
        self.get(endpoint).await
    }

    pub async fn get_ticker(&self, symbol: &str) -> Result<Vec<Ticker>> {
        let endpoint = format!("{}/api/v1/ticker", self.base_url);
        let url = reqwest::Url::parse_with_params(&endpoint, &[("symbol", symbol)])
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn get_order_book_depth(&self, symbol: &str) -> Result<OrderBookDepth> {
        let endpoint = format!("{}/api/v1/depth", self.base_url);
        let url = reqwest::Url::parse_with_params(&endpoint, &[("symbol", symbol)])
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }

    pub async fn get_k_lines(
        &self,
        symbol: &str,
        kline_interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<Kline>> {
        let endpoint = format!("/{}/api/v1/klines", self.base_url);
        let url = reqwest::Url::parse_with_params(
            &endpoint,
            &[
                ("symbol", symbol.to_string()),
                ("interval", kline_interval.to_string()),
                (
                    "start_time",
                    start_time.map(|t| t.to_string()).unwrap_or("".to_string()),
                ),
                (
                    "end_time",
                    end_time.map(|t| t.to_string()).unwrap_or("".to_string()),
                ),
            ],
        )
        .map_err(|e| crate::error::Error::UrlParseError(e.to_string()))?;
        self.get(url).await
    }
}
