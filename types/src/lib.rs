//! Types module for the Backpack Exchange API.
//!
//! This module contains various types used across the Backpack Exchange API,
//! including enums and structs for capital, markets, orders, trades, and user data.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub mod account;
pub mod borrow_lend;
pub mod capital;
pub mod fill;
pub mod futures;
pub mod history;
pub mod margin;
pub mod markets;
pub mod order;
pub mod rfq;
pub mod trade;
pub mod user;
pub mod vault;

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    EnumString,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum Blockchain {
    #[default]
    Solana,
    Ethereum,
    Polygon,
    Bitcoin,
    Internal,
    EqualsMoney,
    Cardano,
    Hyperliquid,
    Story,
    Bsc,
    Dogecoin,
    Sui,
    XRP,
    Litecoin,
    Berachain,
    HyperEVM,
    Plasma,
    Arbitrum,
    Base,
    Optimism,
    Aptos,
    Sei,
    Tron,
    #[strum(serialize = "0G")]
    #[serde(rename = "0G")]
    ZeroG,
    Eclipse,
    Fogo,
    Monad,
    Stable,
    Zcash,
    #[serde(other)]
    Unknown,
}
