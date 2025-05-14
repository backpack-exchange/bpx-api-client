//! Backpack Exchange API Client
//!
//! This module provides the `BpxClient` for interacting with the Backpack Exchange API.
//! It includes functionality for authenticated and public endpoints,
//! along with utilities for error handling, request signing, and response processing.
//!
//! ## Features
//! - Request signing and authentication using ED25519 signatures.
//! - Supports both REST and WebSocket endpoints.
//! - Includes modules for managing capital, orders, trades, and user data.
//!
//! ## Example
//! ```no_run
//! use bpx_api_client::{BACKPACK_API_BASE_URL, BpxClient};
//!
//! #[tokio::main]
//! async fn main() {
//!     let base_url = BACKPACK_API_BASE_URL.to_string();
//!     let secret = "your_api_secret_here";
//!     let headers = None;
//!
//!     let client = BpxClient::init(base_url, secret, headers)
//!         .expect("Failed to initialize Backpack API client");
//!
//!     match client.get_open_orders(Some("SOL_USDC")).await {
//!         Ok(orders) => println!("Open Orders: {:?}", orders),
//!         Err(err) => tracing::error!("Error: {:?}", err),
//!     }
//! }
//! ```

use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use reqwest::{header::CONTENT_TYPE, IntoUrl, Method, Request, Response, StatusCode};
use routes::{
    borrow_lend::API_BORROW_LEND_POSITIONS,
    capital::{API_CAPITAL, API_COLLATERAL, API_DEPOSITS, API_DEPOSIT_ADDRESS, API_WITHDRAWALS},
    futures::API_FUTURES_POSITION,
    order::{API_ORDER, API_ORDERS},
    rfq::{API_RFQ, API_RFQ_QUOTE},
    user::API_USER_2FA,
};
use serde::Serialize;
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

pub mod error;

mod routes;

#[cfg(feature = "ws")]
mod ws;

/// Re-export of the Backpack Exchange API types.
pub use bpx_api_types as types;

/// Re-export of the custom `Error` type and `Result` alias for error handling.
pub use error::{Error, Result};

const API_USER_AGENT: &str = "bpx-rust-client";
const API_KEY_HEADER: &str = "X-API-Key";

const DEFAULT_WINDOW: u32 = 5000;

const SIGNATURE_HEADER: &str = "X-Signature";
const TIMESTAMP_HEADER: &str = "X-Timestamp";
const WINDOW_HEADER: &str = "X-Window";

const JSON_CONTENT: &str = "application/json; charset=utf-8";

/// The official base URL for the Backpack Exchange REST API.
pub const BACKPACK_API_BASE_URL: &str = "https://api.backpack.exchange";

/// The official WebSocket URL for real-time data from the Backpack Exchange.
pub const BACKPACK_WS_URL: &str = "wss://ws.backpack.exchange";

/// Type alias for custom HTTP headers passed to `BpxClient` during initialization.
pub type BpxHeaders = reqwest::header::HeaderMap;

/// A client for interacting with the Backpack Exchange API.
#[derive(Debug, Clone)]
pub struct BpxClient {
    signer: SigningKey,
    verifier: VerifyingKey,
    base_url: String,
    ws_url: Option<String>,
    client: reqwest::Client,
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

// Public functions.
impl BpxClient {
    /// Initializes a new client with the given base URL, API secret, and optional headers.
    ///
    /// This sets up the signing and verification keys, and creates a `reqwest` client
    /// with default headers including the API key and content type.
    pub fn init(base_url: String, secret: &str, headers: Option<BpxHeaders>) -> Result<Self> {
        Self::init_internal(base_url, None, secret, headers)
    }

    /// Initializes a new client with WebSocket support.
    #[cfg(feature = "ws")]
    pub fn init_with_ws(base_url: String, ws_url: String, secret: &str, headers: Option<BpxHeaders>) -> Result<Self> {
        Self::init_internal(base_url, Some(ws_url), secret, headers)
    }

    /// Internal helper function for client initialization.
    fn init_internal(
        base_url: String,
        ws_url: Option<String>,
        secret: &str,
        headers: Option<BpxHeaders>,
    ) -> Result<Self> {
        let signer = STANDARD
            .decode(secret)?
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
            signer,
            verifier,
            base_url,
            ws_url,
            client,
        })
    }

    /// Creates a new, empty `BpxHeaders` instance.
    pub fn create_headers() -> BpxHeaders {
        reqwest::header::HeaderMap::new()
    }

    /// Processes the response to check for HTTP errors and extracts
    /// the response content.
    ///
    /// Returns a custom error if the status code is non-2xx.
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

    /// Sends a GET request to the specified URL and signs it before execution.
    pub async fn get<U: IntoUrl>(&self, url: U) -> Result<Response> {
        let req = self.build_and_maybe_sign_request::<(), _>(url, Method::GET, None)?;
        tracing::debug!("req: {:?}", req);
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Sends a POST request with a JSON payload to the specified URL and signs it.
    pub async fn post<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let req = self.build_and_maybe_sign_request(url, Method::POST, Some(&payload))?;
        tracing::debug!("req: {:?}", req);
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Sends a DELETE request with a JSON payload to the specified URL and signs it.
    pub async fn delete<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let req = self.build_and_maybe_sign_request(url, Method::DELETE, Some(&payload))?;
        tracing::debug!("req: {:?}", req);
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Returns a reference to the `VerifyingKey` used for request verification.
    pub fn verifier(&self) -> &VerifyingKey {
        &self.verifier
    }

    /// Returns a reference to the underlying HTTP client.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

// Private functions.
impl BpxClient {
    /// Signs a request by generating a signature from the request details
    /// and appending necessary headers for authentication.
    ///
    /// # Arguments
    /// * `req` - The mutable reference to the request to be signed.
    fn build_and_maybe_sign_request<P: Serialize, U: IntoUrl>(
        &self,
        url: U,
        method: Method,
        payload: Option<&P>,
    ) -> Result<Request> {
        let url = url.into_url()?;
        let instruction = match url.path() {
            API_CAPITAL if method == Method::GET => "balanceQuery",
            API_DEPOSITS if method == Method::GET => "depositQueryAll",
            API_DEPOSIT_ADDRESS if method == Method::GET => "depositAddressQuery",
            API_WITHDRAWALS if method == Method::GET => "withdrawalQueryAll",
            API_WITHDRAWALS if method == Method::POST => "withdraw",
            API_USER_2FA if method == Method::POST => "issueTwoFactorToken",
            API_ORDER if method == Method::GET => "orderQuery",
            API_ORDER if method == Method::POST => "orderExecute",
            API_ORDER if method == Method::DELETE => "orderCancel",
            API_ORDERS if method == Method::GET => "orderQueryAll",
            API_ORDERS if method == Method::DELETE => "orderCancelAll",
            API_RFQ if method == Method::POST => "rfqSubmit",
            API_RFQ_QUOTE if method == Method::POST => "quoteSubmit",
            API_FUTURES_POSITION if method == Method::GET => "positionQuery",
            API_BORROW_LEND_POSITIONS if method == Method::GET => "borrowLendPositionQuery",
            API_COLLATERAL if method == Method::GET => "collateralQuery",
            _ => {
                let req = self.client().request(method, url);
                if let Some(payload) = payload {
                    return Ok(req.json(payload).build()?);
                } else {
                    return Ok(req.build()?);
                }
            }
        };

        let query_params = url.query_pairs().collect::<BTreeMap<Cow<'_, str>, Cow<'_, str>>>();
        let body_params = if let Some(payload) = payload {
            let s = serde_json::to_value(payload)?;
            match s {
                Value::Object(map) => map
                    .into_iter()
                    .map(|(k, v)| (k, v.to_string()))
                    .collect::<BTreeMap<_, _>>(),
                _ => return Err(Error::InvalidRequest("payload must be a JSON object".to_string())),
            }
        } else {
            BTreeMap::new()
        };

        let timestamp = now_millis();
        let mut signee = format!("instruction={instruction}");
        for (k, v) in query_params {
            signee.push_str(&format!("&{k}={v}"));
        }
        for (k, v) in body_params {
            let v = v.trim_start_matches('"').trim_end_matches('"');
            signee.push_str(&format!("&{k}={v}"));
        }
        signee.push_str(&format!("&timestamp={timestamp}&window={DEFAULT_WINDOW}"));
        tracing::debug!("signee: {}", signee);

        let signature: Signature = self.signer.sign(signee.as_bytes());
        let signature = STANDARD.encode(signature.to_bytes());

        let mut req = self.client().request(method, url);
        if let Some(payload) = payload {
            req = req.json(payload);
        }
        let mut req = req.build()?;
        req.headers_mut().insert(SIGNATURE_HEADER, signature.parse()?);
        req.headers_mut()
            .insert(TIMESTAMP_HEADER, timestamp.to_string().parse()?);
        req.headers_mut()
            .insert(WINDOW_HEADER, DEFAULT_WINDOW.to_string().parse()?);
        if matches!(req.method(), &Method::POST | &Method::DELETE) {
            req.headers_mut().insert(CONTENT_TYPE, JSON_CONTENT.parse()?);
        }
        Ok(req)
    }
}

/// Returns the current time in milliseconds since UNIX epoch.
fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}
