use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
pub use error::{Error, Result};
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, Request, Response, StatusCode};
use serde::Serialize;
use std::collections::BTreeMap;

pub use bpx_api_types as types;
pub use reqwest;

pub mod capital;
pub mod error;
pub mod markets;
pub mod order;
pub mod trades;
pub mod user;

const API_USER_AGENT: &str = "bpx-rust-client";
const API_KEY_HEADER: &str = "X-API-Key";

const SIGNING_WINDOW: u32 = 5000;

const SIGNATURE_HEADER: &str = "X-Signature";
const TIMESTAMP_HEADER: &str = "X-Timestamp";
const WINDOW_HEADER: &str = "X-Window";

const JSON_CONTENT: &str = "application/json; charset=utf-8";

#[derive(Debug, Clone)]
pub struct BpxClient {
    pub verifier: VerifyingKey,
    signer: SigningKey,
    base_url: String,
    pub client: reqwest::Client,
}

impl std::ops::Deref for BpxClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl std::ops::DerefMut for BpxClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl AsRef<reqwest::Client> for BpxClient {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

impl BpxClient {
    /// Initialize a new client with the given base URL, API key, and API secret.
    ///
    /// # Arguments
    /// * `base_url` - The base URL of the API.
    /// * `api_secret` - The API secret.
    /// * `headers` - Additional headers to include in the request.
    ///
    /// # Returns
    /// A new client instance.
    pub fn init(base_url: String, api_secret: &str, headers: Option<reqwest::header::HeaderMap>) -> Result<Self> {
        let signer = STANDARD
            .decode(api_secret)?
            .try_into()
            .map(|s| SigningKey::from_bytes(&s))
            .map_err(|_| Error::SecretKey)?;

        let verifier = signer.verifying_key();

        let mut headers = headers.unwrap_or_default();
        headers.insert(API_KEY_HEADER, STANDARD.encode(verifier).parse()?);
        headers.insert(CONTENT_TYPE, JSON_CONTENT.parse()?);

        let client = reqwest::Client::builder()
            .user_agent(API_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(BpxClient {
            verifier,
            signer,
            base_url,
            client,
        })
    }

    fn sign(&self, req: &mut Request) -> Result<()> {
        let instruction = match req.url().path() {
            "/api/v1/capital" if req.method() == Method::GET => "balanceQuery",
            "/wapi/v1/capital/deposits" if req.method() == Method::GET => "depositQueryAll",
            "/wapi/v1/capital/deposit/address" if req.method() == Method::GET => "depositAddressQuery",
            "/wapi/v1/capital/withdrawals" if req.method() == Method::GET => "withdrawalQueryAll",
            "/wapi/v1/capital/withdrawals" if req.method() == Method::POST => "withdraw",
            "/wapi/v1/user/2fa" if req.method() == Method::POST => "issueTwoFactorToken",
            "/api/v1/order" if req.method() == Method::GET => "orderQuery",
            "/api/v1/order" if req.method() == Method::POST => "orderExecute",
            "/api/v1/order" if req.method() == Method::DELETE => "orderCancel",
            "/api/v1/orders" if req.method() == Method::GET => "orderQueryAll",
            "/api/v1/orders" if req.method() == Method::DELETE => "orderCancelAll",
            _ => return Ok(()), // other endpoints don't require signing
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis();

        let query_params = req
            .url()
            .query_pairs()
            .map(|(x, y)| (x.into_owned(), y.into_owned()))
            .collect::<BTreeMap<String, String>>();

        let body_params = if let Some(b) = req.body() {
            let s = std::str::from_utf8(b.as_bytes().unwrap_or_default())?;
            serde_json::from_str::<BTreeMap<String, String>>(s)?
        } else {
            BTreeMap::new()
        };

        let mut signee = format!("instruction={instruction}");
        for (k, v) in query_params {
            signee.push_str(&format!("&{k}={v}"));
        }
        for (k, v) in body_params {
            signee.push_str(&format!("&{k}={v}"));
        }
        signee.push_str(&format!("&timestamp={timestamp}&window={SIGNING_WINDOW}"));
        tracing::debug!("signee: {}", signee);

        let signature: Signature = self.signer.sign(signee.as_bytes());
        let signature = STANDARD.encode(signature.to_bytes());

        req.headers_mut().insert(SIGNATURE_HEADER, signature.parse()?);
        req.headers_mut().insert(TIMESTAMP_HEADER, timestamp.to_string().parse()?);
        req.headers_mut().insert(WINDOW_HEADER, SIGNING_WINDOW.to_string().parse()?);

        if matches!(req.method(), &Method::POST | &Method::DELETE) {
            req.headers_mut().insert(CONTENT_TYPE, JSON_CONTENT.parse()?);
        }

        Ok(())
    }

    async fn process_response(res: Response) -> Result<Response> {
        if let Err(e) = res.error_for_status_ref() {
            let err_text = res.text().await?;
            let err = Error::BpxApiError {
                status_code: e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                message: err_text,
            };
            return Err(err);
        }
        Ok(res)
    }

    pub async fn get<U: IntoUrl>(&self, url: U) -> Result<Response> {
        let mut req = self.client.get(url).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    pub async fn post<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let mut req = self.client.post(url).json(&payload).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    pub async fn delete<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let mut req = self.client.delete(url).json(&payload).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }
}
