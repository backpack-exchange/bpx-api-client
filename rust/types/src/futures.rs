use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    // pub imf_function: TBD,
    // pub mmf_function: TBD,
    pub break_even_price: String,
    pub cumulative_funding_payment: String,
    pub cumulative_interest: String,
    pub entry_price: String,
    pub est_liquidation_price: String,
    pub imf: String,
    pub mark_price: String,
    pub mmf: String,
    pub net_cost: String,
    pub net_exposure_notional: String,
    pub net_exposure_quantity: String,
    pub net_quantity: String,
    pub pnl_realized: String,
    pub pnl_unrealized: String,
    pub position_id: String,
    pub subaccount_id: Option<u64>,
    pub symbol: String,
    pub user_id: u64,
}
