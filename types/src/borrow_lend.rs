use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::margin::MarginFunction;

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
