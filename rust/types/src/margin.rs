use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginFunction {
    base: String,
    factor: String,
    #[serde(rename = "type")]
    function_type: String,
}
