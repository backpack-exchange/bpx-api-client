use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::margin::MarginFunction;

// {
//   "cumulativeInterest": "0.0001343044",
//   "id": "1532703774",
//   "imf": "0.1",
//   "imfFunction": {
//     "base": "0.1",
//     "factor": "0.00036",
//     "type": "sqrt"
//   },
//   "markPrice": "147.18280534",
//   "mmf": "0.05",
//   "mmfFunction": {
//     "base": "0.05",
//     "factor": "0.000216",
//     "type": "sqrt"
//   },
//   "netExposureNotional": "0.022716253049297736",
//   "netExposureQuantity": "0.0001543404",
//   "netQuantity": "0.0001543404",
//   "symbol": "SOL"
// }

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
