//! These tests are slightly brittle because they connect to the "real" exchange, and thus
//! rely on the exchange being online and accessible from the test environment.
//!
//! However they are useful to prove that our types are serde compatible with the exchange,
//! that URLs are correct, and so on.
//!
//! The tests are feature gated so that they can be run optionally, since if the exchange
//! is down we don't want CI to fail. We can run them in CI as an optional step which is
//! allowed to fail.

#[cfg(feature = "integration-tests")]
mod tests {
    use bpx_api_client::{BpxClient, Result};

    // BTC_USDC might not always be a market, so this makes the tests slightly brittle.
    // We can change this if that ever happens.
    const BTC_USDC: &str = "BTC_USDC";

    mod trades {
        use super::*;

        #[tokio::test]
        async fn test_get_recent_trades() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let _trades = client.get_recent_trades(BTC_USDC, Some(10)).await?;

            Ok(())
        }

        #[tokio::test]
        async fn test_get_historical_trades() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let _trades = client
                .get_historical_trades(BTC_USDC, Some(10), Some(5))
                .await?;

            Ok(())
        }
    }

    mod markets {
        use super::*;
        use std::time::{SystemTime, UNIX_EPOCH};

        // BTC_USDC might not always be a market, so this makes the tests slightly brittle.
        // We can change this if that ever happens.
        const BTC_USDC: &str = "BTC_USDC";
        const BTC_USDC_PERP: &str = "BTC_USDC_PERP";

        fn now_millis() -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64
        }

        #[tokio::test]
        async fn test_get_assets() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let assets = client.get_assets().await?;

            // Should return at least some assets
            assert!(!assets.is_empty());

            Ok(())
        }

        #[tokio::test]
        async fn test_get_markets() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let markets = client.get_markets().await?;

            // Should return at least some markets
            assert!(!markets.is_empty());

            Ok(())
        }

        #[tokio::test]
        async fn test_get_all_mark_prices() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let mark_prices = client.get_all_mark_prices().await?;

            // Should return at least some mark prices
            assert!(!mark_prices.is_empty());

            Ok(())
        }

        #[tokio::test]
        async fn test_get_ticker() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();

            let ticker = client.get_ticker(BTC_USDC).await?;

            // Ticker should have valid data
            assert_eq!(ticker.symbol, BTC_USDC);

            Ok(())
        }

        #[tokio::test]
        async fn test_get_tickers() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let tickers = client.get_tickers().await?;

            // Should return at least some tickers
            assert!(!tickers.is_empty());

            Ok(())
        }

        #[tokio::test]
        async fn test_get_order_book_depth() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let _depth = client.get_order_book_depth(BTC_USDC).await?;

            Ok(())
        }

        #[tokio::test]
        async fn test_get_funding_interval_rates() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();
            let funding_rates = client.get_funding_interval_rates(BTC_USDC_PERP).await?;

            // Should return at least some funding rates
            assert!(!funding_rates.is_empty());

            Ok(())
        }

        #[tokio::test]
        async fn test_get_k_lines() -> Result<()> {
            let client = BpxClient::builder().build().unwrap();

            // Get klines for the last 2 hrs
            let end_time = now_millis() / 1000;
            let start_time = end_time - 3600 * 2; // 2 hour ago
            let klines = client
                .get_k_lines(BTC_USDC, "15m", start_time as i64, Some(end_time as i64))
                .await?;

            // Should return at least some klines
            assert!(!klines.is_empty());

            Ok(())
        }
    }
}
