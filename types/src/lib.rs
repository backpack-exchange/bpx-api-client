//! Types module for the Backpack Exchange API.
//!
//! This module contains various types used across the Backpack Exchange API,
//! including enums and structs for capital, markets, orders, trades, and user data.

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

/// Serde via the strum `Display` / `FromStr` impls, so the wire strings live in one place.
/// Every enum using this has a `#[strum(default)] Unknown(String)` variant, so parsing never
/// fails: an unrecognised value lands in `Unknown` carrying the raw string.
macro_rules! serde_via_strum {
    ($($ty:ty),* $(,)?) => {$(
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.collect_str(self)
            }
        }
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let raw = <std::borrow::Cow<'de, str>>::deserialize(d)?;
                raw.parse().map_err(serde::de::Error::custom)
            }
        }
    )*};
}
pub(crate) use serde_via_strum;

#[derive(Debug, Display, Clone, Default, EnumString, PartialEq, Eq, Hash, EnumIter)]
#[non_exhaustive]
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
    ZeroG,
    Eclipse,
    Fogo,
    Monad,
    Stable,
    Zcash,
    Avalanche,
    BitcoinCash,
    Linea,
    Near,
    Robinhood,
    Tempo,
    /// A blockchain this client version does not know. Carries the wire string.
    #[strum(default)]
    Unknown(String),
}
serde_via_strum!(Blockchain);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::assert_wire;

    #[test]
    fn blockchain_wire() {
        assert_wire(&Blockchain::Solana, "Solana");
        assert_wire(&Blockchain::ZeroG, "0G");
        assert_wire(&Blockchain::HyperEVM, "HyperEVM");
        assert_wire(&Blockchain::XRP, "XRP");
        assert_wire(&Blockchain::BitcoinCash, "BitcoinCash");
        assert_wire(
            &Blockchain::Unknown("SomeFutureChain".into()),
            "SomeFutureChain",
        );
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::fmt::Debug;

    use serde::{Serialize, de::DeserializeOwned};

    /// Asserts that `value` serializes to the JSON string `wire` and that `wire` parses back
    /// to `value`.
    pub(crate) fn assert_wire<T>(value: &T, wire: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json = format!("\"{wire}\"");
        assert_eq!(serde_json::to_string(value).unwrap(), json);
        assert_eq!(serde_json::from_str::<T>(&json).unwrap(), *value);
    }
}
