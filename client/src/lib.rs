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
//! # // We depend on tokio only when the `ws` feature is enabled.
//! # #[cfg(feature = "ws")]
//! # {
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
//! # }
//! ```

use base64ct::{Base64, Encoding};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use reqwest::{IntoUrl, Method, Request, Response, StatusCode, Url, header::CONTENT_TYPE};
use routes::{
    account::{
        API_ACCOUNT, API_ACCOUNT_CONVERT_DUST, API_ACCOUNT_MAX_BORROW, API_ACCOUNT_MAX_ORDER,
        API_ACCOUNT_MAX_WITHDRAWAL,
    },
    borrow_lend::API_BORROW_LEND_POSITIONS,
    capital::{API_CAPITAL, API_COLLATERAL, API_DEPOSIT_ADDRESS, API_DEPOSITS, API_WITHDRAWALS},
    futures::API_FUTURES_POSITION,
    history::API_FILLS_HISTORY,
    order::{API_ORDER, API_ORDERS},
    rfq::{API_RFQ, API_RFQ_QUOTE},
    user::API_USER_2FA,
    vault::API_VAULT_PENDING_REDEEMS,
};
use serde::Serialize;
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub mod error;

mod routes;

#[cfg(feature = "ws")]
mod ws;

/// Re-export of the Backpack Exchange API types.
pub use bpx_api_types as types;

/// Re-export of the custom `Error` type and `Result` alias for error handling.
pub use error::{Error, Result};

use crate::routes::rfq::{API_RFQ_ACCEPT, API_RFQ_CANCEL, API_RFQ_REFRESH};

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
    signing_key: Option<SigningKey>,
    verifying_key: Option<VerifyingKey>,
    base_url: Url,
    #[cfg_attr(not(feature = "ws"), allow(dead_code))]
    ws_url: Url,
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
    pub fn builder() -> BpxClientBuilder {
        BpxClientBuilder::new()
    }

    /// Initializes a new client with the given base URL, API secret, and optional headers.
    ///
    /// This sets up the signing and verification keys, and creates a `reqwest` client
    /// with default headers including the API key and content type.
    pub fn init(base_url: String, secret: &str, headers: Option<BpxHeaders>) -> Result<Self> {
        BpxClientBuilder::new()
            .base_url(base_url)
            .secret(secret)
            .headers(headers.unwrap_or_default())
            .build()
    }

    /// Initializes a new client with WebSocket support.
    #[cfg(feature = "ws")]
    #[deprecated(
        note = "Use BpxClient::builder() instead to configure the client with a custom websocket URL."
    )]
    pub fn init_with_ws(
        base_url: String,
        ws_url: String,
        secret: &str,
        headers: Option<BpxHeaders>,
    ) -> Result<Self> {
        BpxClientBuilder::new()
            .base_url(base_url)
            .ws_url(ws_url)
            .secret(secret)
            .headers(headers.unwrap_or_default())
            .build()
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
                message: err_text.into(),
            };
            return Err(err);
        }
        Ok(res)
    }

    /// Sends a GET request to the specified URL and signs it before execution.
    pub async fn get<U: IntoUrl>(&self, url: U) -> Result<Response> {
        let req = self.build_and_maybe_sign_request::<(), _>(url, Method::GET, None)?;
        tracing::debug!(?req, "GET request");
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Sends a POST request with a JSON payload to the specified URL and signs it.
    pub async fn post<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let req = self.build_and_maybe_sign_request(url, Method::POST, Some(&payload))?;
        tracing::debug!(?req, "POST request");
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Sends a DELETE request with a JSON payload to the specified URL and signs it.
    pub async fn delete<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let req = self.build_and_maybe_sign_request(url, Method::DELETE, Some(&payload))?;
        tracing::debug!(?req, "DELETE request");
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Sends a PATCH request with a JSON payload to the specified URL and signs it.
    pub async fn patch<P: Serialize, U: IntoUrl>(&self, url: U, payload: P) -> Result<Response> {
        let req = self.build_and_maybe_sign_request(url, Method::PATCH, Some(&payload))?;
        tracing::debug!(?req, "PATCH request");
        let res = self.client.execute(req).await?;
        Self::process_response(res).await
    }

    /// Returns a reference to the [`VerifyingKey`] used for request verification.
    /// Return will be [`Some`] if the client was initialised with a secret key, otherwise [`None`].
    pub const fn verifying_key(&self) -> Option<&VerifyingKey> {
        self.verifying_key.as_ref()
    }

    /// Returns a reference to the underlying HTTP client.
    pub const fn client(&self) -> &reqwest::Client {
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
            API_ORDERS if method == Method::POST => "orderExecute",
            API_ORDERS if method == Method::DELETE => "orderCancelAll",
            API_RFQ if method == Method::POST => "rfqSubmit",
            API_RFQ_QUOTE if method == Method::POST => "quoteSubmit",
            API_RFQ_ACCEPT if method == Method::POST => "quoteAccept",
            API_RFQ_CANCEL if method == Method::POST => "rfqCancel",
            API_RFQ_REFRESH if method == Method::POST => "rfqRefresh",
            API_FUTURES_POSITION if method == Method::GET => "positionQuery",
            API_BORROW_LEND_POSITIONS if method == Method::GET => "borrowLendPositionQuery",
            API_COLLATERAL if method == Method::GET => "collateralQuery",
            API_ACCOUNT if method == Method::GET => "accountQuery",
            API_ACCOUNT_MAX_BORROW if method == Method::GET => "maxBorrowQuantity",
            API_ACCOUNT_MAX_ORDER if method == Method::GET => "maxOrderQuantity",
            API_ACCOUNT_MAX_WITHDRAWAL if method == Method::GET => "maxWithdrawalQuantity",
            API_ACCOUNT if method == Method::PATCH => "accountUpdate",
            API_ACCOUNT_CONVERT_DUST if method == Method::POST => "convertDust",
            API_FILLS_HISTORY if method == Method::GET => "fillHistoryQueryAll",
            API_VAULT_PENDING_REDEEMS if method == Method::GET => "vaultPendingRedeemsQuery",
            _ => {
                let req = self.client().request(method, url);
                if let Some(payload) = payload {
                    return Ok(req.json(payload).build()?);
                } else {
                    return Ok(req.build()?);
                }
            }
        };

        let Some(signing_key) = &self.signing_key else {
            return Err(Error::NotAuthenticated);
        };

        let query_params = url
            .query_pairs()
            .collect::<BTreeMap<Cow<'_, str>, Cow<'_, str>>>();

        let mut signee = if let Some(payload) = payload {
            let value = serde_json::to_value(payload)?;
            build_signee_query_and_payload(instruction, value, &query_params)?
        } else {
            build_signee_query(instruction, &query_params)
        };

        let timestamp = now_millis();
        signee.push_str(&format!("&timestamp={timestamp}&window={DEFAULT_WINDOW}"));
        tracing::debug!("signee: {}", signee);

        let signature: Signature = signing_key.sign(signee.as_bytes());
        let signature = Base64::encode_string(&signature.to_bytes());

        let mut req = self.client().request(method, url);
        if let Some(payload) = payload {
            req = req.json(payload);
        }
        let mut req = req.build()?;
        req.headers_mut()
            .insert(SIGNATURE_HEADER, signature.parse()?);
        req.headers_mut()
            .insert(TIMESTAMP_HEADER, timestamp.to_string().parse()?);
        req.headers_mut()
            .insert(WINDOW_HEADER, DEFAULT_WINDOW.to_string().parse()?);
        if matches!(req.method(), &Method::POST | &Method::DELETE) {
            req.headers_mut()
                .insert(CONTENT_TYPE, JSON_CONTENT.parse()?);
        }
        Ok(req)
    }
}

fn build_signee_query_and_payload(
    instruction: &str,
    payload: serde_json::Value,
    query_params: &BTreeMap<Cow<'_, str>, Cow<'_, str>>,
) -> Result<String> {
    match payload {
        Value::Object(map) => {
            let body_params = map
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect::<BTreeMap<_, _>>();
            let mut signee = build_signee_query(instruction, query_params);
            for (k, v) in body_params {
                let v = v.trim_start_matches('"').trim_end_matches('"');
                signee.push_str(&format!("&{k}={v}"));
            }
            Ok(signee)
        }
        Value::Array(array) => array
            .into_iter()
            .map(|item| build_signee_query_and_payload(instruction, item, query_params))
            .collect::<Result<Vec<_>>>()
            .map(|parts| parts.join("&")),
        _ => Err(Error::InvalidRequest(
            "payload must be a JSON object".into(),
        )),
    }
}

fn build_signee_query(
    instruction: &str,
    query_params: &BTreeMap<Cow<'_, str>, Cow<'_, str>>,
) -> String {
    let mut signee = format!("instruction={instruction}");
    for (k, v) in query_params {
        signee.push_str(&format!("&{k}={v}"));
    }
    signee
}

#[derive(Debug, Default)]
pub struct BpxClientBuilder {
    base_url: Option<String>,
    ws_url: Option<String>,
    secret: Option<String>,
    headers: Option<BpxHeaders>,
    timeout: Option<u64>,
}

impl BpxClientBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the base URL for the Backpack Exchange API.
    /// If not set, defaults to `BACKPACK_API_BASE_URL`.
    ///
    /// # Arguments
    /// * `base_url` - The base URL
    ///
    /// # Returns
    /// * `Self` - The updated builder instance
    pub fn base_url(mut self, base_url: impl ToString) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    /// Sets the WebSocket URL for the Backpack Exchange API.
    /// If not set, defaults to `BACKPACK_WS_URL`.
    ///
    /// # Arguments
    /// * `ws_url` - The WebSocket URL
    ///
    /// # Returns
    /// * `Self` - The updated builder instance
    #[cfg(feature = "ws")]
    pub fn ws_url(mut self, ws_url: impl ToString) -> Self {
        self.ws_url = Some(ws_url.to_string());
        self
    }

    /// Sets the API secret for signing requests.
    /// If not set, the client will be unauthenticated.
    ///
    /// # Arguments
    /// * `secret` - The API secret
    ///
    /// # Returns
    /// * `Self` - The updated builder instance
    pub fn secret(mut self, secret: impl ToString) -> Self {
        self.secret = Some(secret.to_string());
        self
    }

    /// Sets custom HTTP headers for the client.
    /// If not set, no additional headers will be included.
    ///
    /// # Arguments
    /// * `headers` - The custom HTTP headers
    ///
    /// # Returns
    /// * `Self` - The updated builder instance
    pub fn headers(mut self, headers: BpxHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Sets a custom Timeout for the underlying http client
    /// If not set, a default of 30 seconds is used.
    ///
    /// # Arguments
    /// * `timeout` - The timeout in seconds
    ///
    /// # Returns
    /// * `Self` - The updated builder instance
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Builds the `BpxClient` instance with the configured parameters.
    ///
    /// # Returns
    /// * `Result<BpxClient>` - The constructed client or an error if building fails
    pub fn build(self) -> Result<BpxClient> {
        let base_url = self.base_url.as_deref().unwrap_or(BACKPACK_API_BASE_URL);
        let base_url = Url::parse(base_url)?;

        let ws_url = self.ws_url.as_deref().unwrap_or(BACKPACK_WS_URL);
        let ws_url = Url::parse(ws_url)?;

        let signing_key = if let Some(secret) = self.secret {
            Some(
                Base64::decode_vec(&secret)?
                    .try_into()
                    .map(|s| SigningKey::from_bytes(&s))
                    .map_err(|_| Error::SecretKey)?,
            )
        } else {
            None
        };
        let verifying_key = signing_key.as_ref().map(|s| s.verifying_key());

        let mut header_map = BpxHeaders::new();
        if let Some(headers) = self.headers {
            header_map.extend(headers);
        }

        header_map.insert(CONTENT_TYPE, JSON_CONTENT.parse()?);
        if let Some(signing_key) = &signing_key {
            let verifier = signing_key.verifying_key();
            header_map.insert(
                API_KEY_HEADER,
                Base64::encode_string(&verifier.to_bytes()).parse()?,
            );
        }

        let client = BpxClient {
            signing_key,
            verifying_key,
            base_url,
            ws_url,
            client: reqwest::Client::builder()
                .user_agent(API_USER_AGENT)
                .default_headers(header_map)
                .timeout(Duration::from_secs(self.timeout.unwrap_or(30)))
                .build()?,
        };

        Ok(client)
    }
}

/// Returns the current time in milliseconds since UNIX epoch.
fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}
