use bpx_api_types::user::{RequestTwoFactorPayload, RequestTwoFactorResponse};

use crate::{error::Result, BpxClient};

impl BpxClient {
    pub async fn request_two_factor(
        &self,
        payload: RequestTwoFactorPayload,
    ) -> Result<RequestTwoFactorResponse> {
        let endpoint = format!("{}/wapi/v1/user/2fa", self.base_url);
        let res = self.post(endpoint, payload).await?;

        let data: RequestTwoFactorResponse = res.json().await?;
        Ok(data)
    }
}
