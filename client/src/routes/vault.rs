use crate::error::Result;

use bpx_api_types::vault::{
    Vault, VaultHistory, VaultHistoryParams, VaultMintRequest, VaultRedeem,
    VaultRedeemCancelRequest, VaultRedeemRequest,
};

use crate::BpxClient;

#[doc(hidden)]
pub const API_VAULTS: &str = "/api/v1/vaults";
#[doc(hidden)]
pub const API_VAULT_MINT: &str = "/api/v1/vault/mint";
#[doc(hidden)]
pub const API_VAULT_REDEEM: &str = "/api/v1/vault/redeem";
#[doc(hidden)]
pub const API_VAULT_PENDING_REDEEMS: &str = "/api/v1/vault/redeems/pending";
#[doc(hidden)]
pub const API_VAULTS_HISTORY: &str = "/api/v1/vaults/history";

impl BpxClient {
    /// Fetches information about all available vaults on the exchange.
    pub async fn get_vaults(&self) -> Result<Vec<Vault>> {
        let url = self.base_url.join(API_VAULTS)?;
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Mints vault tokens by depositing an asset into a vault.
    pub async fn vault_mint(&self, request: VaultMintRequest) -> Result<()> {
        let url = self.base_url.join(API_VAULT_MINT)?;
        let res = self.post(url, request).await?;
        let _ = res.bytes().await?;
        Ok(())
    }

    /// Submits a request to redeem vault tokens for USDC.
    pub async fn vault_redeem(&self, request: VaultRedeemRequest) -> Result<()> {
        let url = self.base_url.join(API_VAULT_REDEEM)?;
        let res = self.post(url, request).await?;
        let _ = res.bytes().await?;
        Ok(())
    }

    /// Cancels a pending redeem request for a vault.
    pub async fn vault_redeem_cancel(&self, request: VaultRedeemCancelRequest) -> Result<()> {
        let url = self.base_url.join(API_VAULT_REDEEM)?;
        let res = self.delete(url, request).await?;
        let _ = res.bytes().await?;
        Ok(())
    }

    /// Fetches pending redeem requests for a vault.
    pub async fn get_vault_pending_redeems(&self, vault_id: u32) -> Result<Vec<VaultRedeem>> {
        let mut url = self.base_url.join(API_VAULT_PENDING_REDEEMS)?;
        url.query_pairs_mut()
            .append_pair("vaultId", &vault_id.to_string());
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches historical vault data (NAV, equity, circulating supply).
    pub async fn get_vault_history(&self, params: VaultHistoryParams) -> Result<Vec<VaultHistory>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_VAULTS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
