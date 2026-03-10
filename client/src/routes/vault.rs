use crate::error::Result;

use bpx_api_types::vault::{
    Vault, VaultHistory, VaultHistoryParams, VaultMint, VaultMintHistoryParams, VaultMintRequest,
    VaultRedeem, VaultRedeemCancelRequest, VaultRedeemHistoryParams, VaultRedeemRequest,
};

use crate::BpxClient;

#[doc(hidden)]
pub const API_VAULTS: &str = "/api/v1/vaults";
#[doc(hidden)]
pub const API_VAULT_MINT: &str = "/api/v1/vault/mint";
#[doc(hidden)]
pub const API_VAULT_REDEEM: &str = "/api/v1/vault/redeem";
#[doc(hidden)]
pub const API_VAULTS_HISTORY: &str = "/api/v1/vaults/history";
#[doc(hidden)]
pub const API_VAULT_MINTS_HISTORY: &str = "/wapi/v1/history/vault/mint";
#[doc(hidden)]
pub const API_VAULT_REDEEMS_HISTORY: &str = "/wapi/v1/history/vault/redeem";

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

    /// Fetches historical vault data (NAV, equity, circulating supply).
    pub async fn get_vault_history(&self, params: VaultHistoryParams) -> Result<Vec<VaultHistory>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_VAULTS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches vault mint history (authenticated).
    pub async fn get_vault_mints(&self, params: VaultMintHistoryParams) -> Result<Vec<VaultMint>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_VAULT_MINTS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches vault redeem history (authenticated).
    pub async fn get_vault_redeems(
        &self,
        params: VaultRedeemHistoryParams,
    ) -> Result<Vec<VaultRedeem>> {
        let query_string = serde_qs::to_string(&params)
            .map_err(|e| crate::error::Error::UrlParseError(e.to_string().into_boxed_str()))?;
        let mut url = self.base_url.join(API_VAULT_REDEEMS_HISTORY)?;
        url.set_query(Some(&query_string));
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
