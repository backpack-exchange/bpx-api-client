use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub mod capital;
pub mod markets;
pub mod order;
pub mod trade;

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
}
