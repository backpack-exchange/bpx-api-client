use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::margin::MarginFunction;

/// Summary of a borrow/lend market.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendMarket {
    /// State of the borrow/lend market (e.g. `Open`, `Closed`, `RepayOnly`).
    pub state: String,
    /// Mark price of the spot instrument.
    pub asset_mark_price: Decimal,
    /// The rate borrowers pay.
    pub borrow_interest_rate: Decimal,
    /// The amount of assets borrowed from the pool.
    pub borrowed_quantity: Decimal,
    /// The fee that the exchange takes on borrow/lend yield.
    pub fee: Decimal,
    /// The APY rate lenders receive.
    pub lend_interest_rate: Decimal,
    /// The amount of assets lent to the pool.
    pub lent_quantity: Decimal,
    /// The max utilization that can be used by borrowing or redeeming lend.
    pub max_utilization: Decimal,
    /// Maximum total open borrows/lends.
    pub open_borrow_lend_limit: Decimal,
    /// The optimal utilization rate for the interest rate model.
    pub optimal_utilization: Decimal,
    /// The asset symbol.
    pub symbol: String,
    /// Timestamp of the summary.
    pub timestamp: DateTime<Utc>,
    /// The threshold that triggers borrow throttling.
    pub throttle_utilization_threshold: Decimal,
    /// The max utilization threshold for any given timestep.
    pub throttle_utilization_bound: Decimal,
    /// Max utilization increase per timestep.
    pub throttle_update_fraction: Decimal,
    /// Pool utilization.
    pub utilization: Decimal,
    /// Minimum quantity increment.
    pub step_size: Decimal,
}

/// Historical snapshot of a borrow/lend market.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendHistory {
    /// The rate borrowers pay.
    pub borrow_interest_rate: Decimal,
    /// The amount of assets borrowed from the pool.
    pub borrowed_quantity: Decimal,
    /// The APY rate lenders receive.
    pub lend_interest_rate: Decimal,
    /// The amount of assets lent to the pool.
    pub lent_quantity: Decimal,
    /// Timestamp of the snapshot.
    pub timestamp: DateTime<Utc>,
    /// Pool utilization.
    pub utilization: Decimal,
}

/// Parameters for fetching borrow/lend market history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendMarketHistoryParams {
    /// Time interval for historical data (e.g. `1d`, `1w`, `1month`, `1year`).
    pub interval: String,
    /// Optional asset symbol to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Borrow/lend APY rate for a single asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendApyRate {
    pub symbol: String,
    pub borrow_rate: Decimal,
    pub lend_rate: Decimal,
}

/// Staking APY rate for a single asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingApyRate {
    pub symbol: String,
    pub dilution_factor: Decimal,
    pub staking_rate: Decimal,
}

/// Combined borrow/lend and staking APY rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApyRates {
    pub borrow_lend: Vec<BorrowLendApyRate>,
    pub staking: Vec<StakingApyRate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendPosition {
    pub cumulative_interest: Decimal,
    pub id: String,
    pub symbol: String,
    pub imf: Decimal,
    pub imf_function: MarginFunction,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub mmf_function: MarginFunction,
    pub net_exposure_notional: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_quantity: Decimal,
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_borrow_lend_market_parse() {
        let data = r#"
{
  "state": "Open",
  "symbol": "USDC",
  "timestamp": "2026-07-30T00:53:00.000Z",
  "assetMarkPrice": "1.0001",
  "borrowInterestRate": "0.0523",
  "lendInterestRate": "0.0470",
  "borrowedQuantity": "1500000.50",
  "lentQuantity": "5000000.00",
  "utilization": "0.30",
  "fee": "0.001",
  "maxUtilization": "1",
  "openBorrowLendLimit": "10000000",
  "optimalUtilization": "0.9",
  "stepSize": "0.0000000001",
  "throttleUtilizationThreshold": "0.92",
  "throttleUtilizationBound": "0.95",
  "throttleUpdateFraction": "0.1"
}
        "#;

        let market: BorrowLendMarket = serde_json::from_str(data).unwrap();
        assert_eq!(market.state, "Open");
        assert_eq!(market.symbol, "USDC");
        assert_eq!(market.borrow_interest_rate, dec!(0.0523));
        assert_eq!(market.lend_interest_rate, dec!(0.0470));
        assert_eq!(market.utilization, dec!(0.30));
    }

    #[test]
    fn test_borrow_lend_market_unknown_state() {
        let data = r#"
{
  "state": "SomeFutureState",
  "symbol": "USDC",
  "timestamp": "2026-07-30T00:53:00.000Z",
  "assetMarkPrice": "1",
  "borrowInterestRate": "0",
  "lendInterestRate": "0",
  "borrowedQuantity": "0",
  "lentQuantity": "0",
  "utilization": "0",
  "fee": "0",
  "maxUtilization": "1",
  "openBorrowLendLimit": "0",
  "optimalUtilization": "0.9",
  "stepSize": "0.0000000001",
  "throttleUtilizationThreshold": "0.92",
  "throttleUtilizationBound": "0.95",
  "throttleUpdateFraction": "0.1"
}
        "#;

        let market: BorrowLendMarket = serde_json::from_str(data).unwrap();
        assert_eq!(market.state, "SomeFutureState");
    }

    #[test]
    fn test_borrow_lend_history_parse() {
        let data = r#"
{
  "borrowInterestRate": "0.051",
  "lendInterestRate": "0.046",
  "borrowedQuantity": "1400000",
  "lentQuantity": "4800000",
  "utilization": "0.29",
  "timestamp": "2026-07-29T00:00:00.000Z"
}
        "#;

        let history: BorrowLendHistory = serde_json::from_str(data).unwrap();
        assert_eq!(history.borrow_interest_rate, dec!(0.051));
        assert_eq!(history.utilization, dec!(0.29));
    }

    #[test]
    fn test_apy_rates_parse() {
        let data = r#"
{
  "borrowLend": [
    { "symbol": "USDC", "borrowRate": "0.0523", "lendRate": "0.0470" },
    { "symbol": "BTC", "borrowRate": "0.0312", "lendRate": "0.0265" }
  ],
  "staking": [
    { "symbol": "SOL", "dilutionFactor": "0.85", "stakingRate": "0.0612" },
    { "symbol": "USDC", "dilutionFactor": "0.90", "stakingRate": "0.0480" }
  ]
}
        "#;

        let rates: ApyRates = serde_json::from_str(data).unwrap();
        assert_eq!(rates.borrow_lend.len(), 2);
        assert_eq!(rates.borrow_lend[0].symbol, "USDC");
        assert_eq!(rates.staking.len(), 2);
        assert_eq!(rates.staking[0].symbol, "SOL");
    }

    #[test]
    fn test_borrow_lend_market_history_interval_query() {
        let params = BorrowLendMarketHistoryParams {
            interval: "1d".to_string(),
            symbol: Some("USDC".to_string()),
        };
        let query = serde_qs::to_string(&params).unwrap();
        assert_eq!(query, "interval=1d&symbol=USDC");
    }
}
