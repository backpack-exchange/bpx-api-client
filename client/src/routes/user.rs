use bpx_api_types::user::{RequestTwoFactorPayload, RequestTwoFactorResponse};

use crate::{BpxClient, error::Result};

#[doc(hidden)]
pub const API_USER_2FA: &str = "/wapi/v1/user/2fa";

impl BpxClient {
    /// Requests a two-factor authentication token.
    ///
    /// Sends a request to initiate the two-factor authentication process
    /// with the provided payload and returns the response.
    pub async fn request_two_factor(
        &self,
        payload: RequestTwoFactorPayload,
    ) -> Result<RequestTwoFactorResponse> {
        let endpoint = self.base_url.join(API_USER_2FA)?;
        let res = self.post(endpoint, payload).await?;

        let data: RequestTwoFactorResponse = res.json().await?;
        Ok(data)
    }
}
