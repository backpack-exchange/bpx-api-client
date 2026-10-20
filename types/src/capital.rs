use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

use crate::Blockchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub available: Decimal,
    pub locked: Decimal,
    pub staked: Decimal,
}

impl Balance {
    pub fn total(&self) -> Decimal {
        self.available + self.locked + self.staked
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub id: i32,
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub confirmation_block_number: Option<i32>,
    pub identifier: Option<String>,
    pub transaction_hash: Option<String>,
    pub source: DepositSource,
    pub status: DepositStatus,
    pub subaccount_id: Option<u64>,
    pub symbol: String,
    pub quantity: Decimal,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(
    Debug, Display, Clone, EnumString, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum DepositSource {
    Administrator,
    // Blockchains.
    #[strum(serialize = "0G")]
    ZeroG,
    Aptos,
    Arbitrum,
    Avalanche,
    Base,
    Berachain,
    Bitcoin,
    BitcoinCash,
    Bsc,
    Cardano,
    Dogecoin,
    Eclipse,
    Ethereum,
    Fogo,
    #[strum(serialize = "hyperEVM")]
    HyperEVM,
    Hyperliquid,
    Linea,
    Litecoin,
    Monad,
    Near,
    Polygon,
    Optimism,
    Plasma,
    Robinhood,
    Sei,
    Stable,
    Sui,
    Solana,
    Story,
    Tempo,
    Tron,
    #[strum(serialize = "xRP")]
    XRP,
    Zcash,
    // Payment processors.
    EqualsMoney,
    Banxa,
    Moonpay,
    Onramper,
    // Internal transfer.
    Internal,
    /// A source this client version does not know. Carries the wire string.
    #[strum(default)]
    Unknown(String),
}

#[derive(
    Debug, Display, Clone, EnumString, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum DepositStatus {
    /// Waiting for confirmation, compliance or processing.
    Pending,
    /// Confirmed deposit.
    Confirmed,
    /// Cancelled deposit.
    Cancelled,
    /// Declined payment. Fiat deposits only.
    Declined,
    /// Payment expired. Fiat deposits only.
    Expired,
    /// Payment initiated.
    Initiated,
    /// Payment refunded.
    Refunded,
    /// Ownership verification required.
    OwnershipVerificationRequired,
    /// Sender verification required.
    SenderVerificationRequired,
    /// A status this client version does not know. Carries the wire string.
    #[strum(default)]
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawalPayload {
    pub address: String,
    pub blockchain: Blockchain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub quantity: Decimal,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_redeem: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub id: i32,
    pub blockchain: Blockchain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    pub quantity: Decimal,
    pub fee: Decimal,
    pub symbol: String,
    pub status: WithdrawalStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<u64>,
    pub to_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(
    Debug, Display, Clone, EnumString, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum WithdrawalStatus {
    /// Waiting for compliance, signing, confirmation or processing.
    Pending,
    /// Confirmed withdrawal.
    Confirmed,
    /// Voided by administrators.
    Void,
    /// Ownership verification required.
    OwnershipVerificationRequired,
    /// A status this client version does not know. Carries the wire string.
    #[strum(default)]
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    pub assets_value: Decimal,
    pub borrow_liability: Decimal,
    pub collateral: Vec<CollateralItem>,
    pub imf: Decimal,
    pub unsettled_equity: Decimal,
    pub liabilities_value: Decimal,
    pub margin_fraction: Option<Decimal>,
    pub mmf: Decimal,
    pub net_equity: Decimal,
    pub net_equity_available: Decimal,
    pub net_equity_locked: Decimal,
    pub net_exposure_futures: Decimal,
    pub pnl_unrealized: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralItem {
    pub symbol: String,
    pub asset_mark_price: Decimal,
    pub total_quantity: Decimal,
    pub balance_notional: Decimal,
    pub collateral_weight: Decimal,
    pub collateral_value: Decimal,
    pub open_order_quantity: Decimal,
    pub lend_quantity: Decimal,
    pub available_quantity: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::assert_wire;

    #[test]
    fn deposit_source_wire() {
        assert_wire(&DepositSource::Administrator, "administrator");
        assert_wire(&DepositSource::ZeroG, "0G");
        assert_wire(&DepositSource::HyperEVM, "hyperEVM");
        assert_wire(&DepositSource::XRP, "xRP");
        assert_wire(&DepositSource::BitcoinCash, "bitcoinCash");
        assert_wire(&DepositSource::EqualsMoney, "equalsMoney");
        assert_wire(&DepositSource::Banxa, "banxa");
        assert_wire(&DepositSource::Moonpay, "moonpay");
        assert_wire(&DepositSource::Onramper, "onramper");
        assert_wire(&DepositSource::Internal, "internal");
        assert_wire(
            &DepositSource::Unknown("someFutureSource".into()),
            "someFutureSource",
        );
    }

    #[test]
    fn deposit_status_wire() {
        assert_wire(&DepositStatus::Confirmed, "confirmed");
        assert_wire(
            &DepositStatus::OwnershipVerificationRequired,
            "ownershipVerificationRequired",
        );
        assert_wire(
            &DepositStatus::SenderVerificationRequired,
            "senderVerificationRequired",
        );
        assert_wire(
            &DepositStatus::Unknown("someFutureStatus".into()),
            "someFutureStatus",
        );
    }

    #[test]
    fn withdrawal_status_wire() {
        assert_wire(&WithdrawalStatus::Void, "void");
        assert_wire(
            &WithdrawalStatus::OwnershipVerificationRequired,
            "ownershipVerificationRequired",
        );
        assert_wire(
            &WithdrawalStatus::Unknown("someFutureStatus".into()),
            "someFutureStatus",
        );
    }
}
