use std::collections::HashMap;

use bpx_api_types::markets::{FundingRate, Kline, Market, OrderBookDepth, Ticker, Token};

use crate::error::Result;
use crate::BpxClient;

const API_ASSETS: &str = "/api/v1/assets";
const API_MARKETS: &str = "/api/v1/markets";
const API_TICKER: &str = "/api/v1/ticker";
const API_TICKERS: &str = "/api/v1/tickers";
const API_DEPTH: &str = "/api/v1/depth";
const API_KLINES: &str = "/api/v1/klines";
const API_FUNDING: &str = "/api/v1/fundingRates";

impl BpxClient {
    /// Fetches available assets and their associated tokens.
    pub async fn get_assets(&self) -> Result<HashMap<String, Vec<Token>>> {
        let url = format!("{}{}", self.base_url, API_ASSETS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves a list of available markets.
    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}{}", self.base_url, API_MARKETS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for a given symbol.
    pub async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let url = format!("{}{}?symbol={}", self.base_url, API_TICKER, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for all symbols.
    pub async fn get_tickers(&self) -> Result<Vec<Ticker>> {
        let url = format!("{}{}", self.base_url, API_TICKERS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves the order book depth for a given symbol.
    pub async fn get_order_book_depth(&self, symbol: &str) -> Result<OrderBookDepth> {
        let url = format!("{}{}?symbol={}", self.base_url, API_DEPTH, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Funding interval rate history for futures.
    pub async fn get_funding_interval_rates(&self, symbol: &str) -> Result<Vec<FundingRate>> {
        let url = format!("{}{}?symbol={}", self.base_url, API_FUNDING, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches historical K-line (candlestick) data for a given symbol and interval.
    pub async fn get_k_lines(
        &self,
        symbol: &str,
        kline_interval: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<Kline>> {
        let mut url = format!(
            "/{}{}?symbol={}&kline_interval={}",
            self.base_url, API_KLINES, symbol, kline_interval
        );
        for (k, v) in [("start_time", start_time), ("end_time", end_time)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
