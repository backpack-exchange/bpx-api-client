use serde::{Deserialize, Serialize};

pub mod capital;
pub mod markets;
pub mod order;
pub mod trade;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum Blockchain {
    #[default]
    Solana,
    Ethereum,
    Polygon,
    Bitcoin,
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Blockchain::Solana => write!(f, "Solana"),
            Blockchain::Ethereum => write!(f, "Ethereum"),
            Blockchain::Polygon => write!(f, "Polygon"),
            Blockchain::Bitcoin => write!(f, "Bitcoin"),
        }
    }
}
