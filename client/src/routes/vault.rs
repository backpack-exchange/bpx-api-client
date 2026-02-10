use crate::error::Result;

use bpx_api_types::vault::VaultRedeem;

use crate::BpxClient;

#[doc(hidden)]
pub const API_VAULT_PENDING_REDEEMS: &str = "/api/v1/vault/redeems/pending";

impl BpxClient {
    /// Fetches pending redeem requests for a vault.
    pub async fn get_vault_pending_redeems(
        &self,
        vault_id: u32,
    ) -> Result<Vec<VaultRedeem>> {
        let mut url = self.base_url.join(API_VAULT_PENDING_REDEEMS)?;
        url.query_pairs_mut()
            .append_pair("vaultId", &vault_id.to_string());
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
