use bpx_api_types::markets::{
    Asset, FundingRate, Kline, MarkPrice, Market, OrderBookDepth, OrderBookDepthLimit, Ticker,
};
use reqwest::Method;
use std::borrow::Cow;

use crate::error::Result;
use crate::{BpxClient, BpxClientRequest, Error};

const API_ASSETS: &str = "/api/v1/assets";
const API_MARKETS: &str = "/api/v1/markets";
const API_TICKER: &str = "/api/v1/ticker";
const API_TICKERS: &str = "/api/v1/tickers";
const API_DEPTH: &str = "/api/v1/depth";
const API_KLINES: &str = "/api/v1/klines";
const API_FUNDING: &str = "/api/v1/fundingRates";
const API_MARK_PRICES: &str = "/api/v1/markPrices";

impl BpxClient {
    /// Fetches available assets and their associated tokens.
    pub async fn get_assets(&self) -> Result<Vec<Asset>> {
        let url = self.base_url.join(API_ASSETS)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves a list of available spot and perp markets.
    ///
    /// Note: If you want to retrieve other market types or filter by market type, use `GetMarketsRequest`.
    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        GetMarketsRequest::new()
            .with_spot_markets()
            .with_perp_markets()
            .send(self)
            .await
    }

    // pub async fn get_markets_with_filter

    /// Retrieves mark price, index price and the funding rate for the current interval for all symbols, or the symbol specified.
    pub async fn get_all_mark_prices(&self) -> Result<Vec<MarkPrice>> {
        let url = self.base_url.join(API_MARK_PRICES)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for a given symbol.
    pub async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let mut url = self.base_url.join(API_TICKER)?;
        url.query_pairs_mut().append_pair("symbol", symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for all symbols.
    pub async fn get_tickers(&self) -> Result<Vec<Ticker>> {
        let url = self.base_url.join(API_TICKERS)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves the order book depth for a given symbol.
    pub async fn get_order_book_depth(
        &self,
        symbol: &str,
        limit: Option<OrderBookDepthLimit>,
    ) -> Result<OrderBookDepth> {
        let mut url = self.base_url.join(API_DEPTH)?;
        url.query_pairs_mut().append_pair("symbol", symbol);
        if let Some(limit) = limit {
            url.query_pairs_mut().append_pair("limit", limit.as_ref());
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Funding interval rate history for futures.
    pub async fn get_funding_interval_rates(&self, symbol: &str) -> Result<Vec<FundingRate>> {
        let mut url = self.base_url.join(API_FUNDING)?;
        url.query_pairs_mut().append_pair("symbol", symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches historical K-line (candlestick) data for a given symbol and interval.
    pub async fn get_k_lines(
        &self,
        symbol: &str,
        kline_interval: &str,
        start_time: i64,
        end_time: Option<i64>,
    ) -> Result<Vec<Kline>> {
        let mut url = self.base_url.join(API_KLINES)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", symbol);
            query.append_pair("interval", kline_interval);
            query.append_pair("startTime", &start_time.to_string());
            if let Some(end_time) = end_time {
                query.append_pair("endTime", &end_time.to_string());
            }
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetMarketsRequest(Vec<String>);

impl GetMarketsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_spot_markets(mut self) -> Self {
        self.0.push("SPOT".to_string());
        self
    }

    pub fn with_perp_markets(mut self) -> Self {
        self.0.push("PERP".to_string());
        self
    }

    pub fn with_prediction_markets(mut self) -> Self {
        self.0.push("PREDICTION".to_string());
        self
    }

    pub async fn send(self, client: &BpxClient) -> Result<Vec<Market>> {
        if self.0.is_empty() {
            return Err(Error::InvalidRequest(
                "at least one market type must be specified".into(),
            ));
        }

        let mut url = client.base_url.join(API_MARKETS)?;
        for market_type in self.0 {
            url.query_pairs_mut()
                .append_pair("marketType", &market_type);
        }

        let res = client.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}

impl BpxClientRequest for GetMarketsRequest {
    const PATH: &'static str = API_MARKETS;
    const METHOD: Method = Method::GET;
    type Body = ();

    fn query_params(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut params = Vec::new();
        for market_type in &self.0 {
            params.push((
                Cow::Borrowed("marketType"),
                Cow::Borrowed(market_type.as_str()),
            ));
        }
        params
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }

    fn validate(&self) -> Result<()> {
        if self.0.is_empty() {
            return Err(Error::InvalidRequest(
                "at least one market type must be specified".into(),
            ));
        }

        Ok(())
    }
}
