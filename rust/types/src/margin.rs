use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginFunction {
    pub base: Decimal,
    pub factor: Decimal,
    #[serde(rename = "type")]
    pub function_type: String,
}
