use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestTwoFactorPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestTwoFactorResponse {
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    pub organization_id: i32,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub spot_fee_tier_id: i32,
    pub futures_fee_tier_id: i32,
    pub two_factor_verified: bool,
    pub kyc_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_alias: Option<String>,
    pub show_in_leaderboard: bool,
    pub rewards_multiplier: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_approval_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_contact_platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_contact_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_kyc_session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_kyc_session_trading: Option<String>,
    pub is_safe_enabled: bool,
    pub is_prediction_market_enabled: bool,
    pub eu_claim_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_limit: Option<i16>,
    pub has_passkey: bool,
    pub permissions: UserPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_market_maker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissions {
    pub is_borrow_lend_enabled: bool,
    pub is_crypto_deposit_enabled: bool,
    pub is_crypto_withdrawal_enabled: bool,
    pub is_fiat_deposit_enabled: bool,
    pub is_fiat_withdrawal_enabled: bool,
    pub is_perp_enabled: bool,
    pub is_prediction_enabled: bool,
    pub is_rfq_request_enabled: bool,
    pub is_rfq_quote_enabled: bool,
    pub is_spot_enabled: bool,
    pub leverage_limit: Decimal,
}
