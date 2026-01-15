use bpx_api_types::user::{RequestTwoFactorPayload, RequestTwoFactorResponse, User};

use crate::{BpxClient, error::Result};

#[doc(hidden)]
pub const API_USER: &str = "/wapi/v1/user";
#[doc(hidden)]
pub const API_USER_2FA: &str = "/wapi/v1/user/2fa";

impl BpxClient {
    /// Get user.
    pub async fn get_user(&self) -> Result<User> {
        let endpoint = self.base_url.join(API_USER)?;
        let res = self.get(endpoint).await?;

        let data: User = res.json().await?;
        Ok(data)
    }

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
