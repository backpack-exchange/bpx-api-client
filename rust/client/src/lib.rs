use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
pub use error::{Error, Result};
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, Request, Response};
use serde::Serialize;
use std::collections::BTreeMap;

pub use bpx_api_types as types;

pub mod capital;
pub mod error;
pub mod markets;
pub mod order;
pub mod trades;

const SIGNING_WINDOW: u32 = 5000;

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
    pub fn init(base_url: String, api_key: &str, api_secret: &str) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-Key", api_key.parse()?);
        headers.insert(CONTENT_TYPE, "application/json; charset=utf-8".parse()?);

        let client = reqwest::Client::builder()
            .user_agent("bpx-rust-client")
            .default_headers(headers)
            .build()?;

        let api_secret = STANDARD
            .decode(api_secret)?
            .try_into()
            .map_err(|_| Error::SecretKey)?;

        let signer = SigningKey::from_bytes(&api_secret);
        let verifier = signer.verifying_key();

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
            "/wapi/v1/capital/deposit/address" if req.method() == Method::GET => {
                "depositAddressQuery"
            }
            "/wapi/v1/capital/withdrawals" if req.method() == Method::GET => "withdrawalQueryAll",
            "/wapi/v1/capital/withdrawals" if req.method() == Method::POST => "withdraw",
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

        req.headers_mut()
            .insert("X-Timestamp", timestamp.to_string().parse()?);
        req.headers_mut()
            .insert("X-Window", SIGNING_WINDOW.to_string().parse()?);
        req.headers_mut().insert("X-Signature", signature.parse()?);

        if matches!(req.method(), &Method::POST | &Method::DELETE) {
            req.headers_mut()
                .insert(CONTENT_TYPE, "application/json; charset=utf-8".parse()?);
        }

        Ok(())
    }

    pub async fn get<U: IntoUrl>(&self, url: U) -> Result<Response> {
        let mut req = self.client.get(url).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        self.client.execute(req).await.map_err(Error::from)
    }

    pub async fn post<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let mut req = self.client.post(url).json(&payload).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        self.client.execute(req).await.map_err(Error::from)
    }

    pub async fn delete<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let mut req = self.client.delete(url).json(&payload).build()?;
        tracing::debug!("req: {:?}", req);
        self.sign(&mut req)?;
        self.client.execute(req).await.map_err(Error::from)
    }
}
