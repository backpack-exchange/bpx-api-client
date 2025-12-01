//! Error handling module for the Backpack Exchange API client.
//!
//! Defines a custom `Error` type and a `Result` type alias to encapsulate
//! various errors that can occur during API interactions.

/// A type alias for `Result` using the custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Enum representing possible errors in the Backpack Exchange API client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error decoding a base64 string.
    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] base64ct::Error),

    /// Backpack API returned an error with status code and message.
    #[error("Backpack API error: {status_code}: {message}")]
    BpxApiError {
        status_code: reqwest::StatusCode,
        message: Box<str>,
    },

    /// Invalid HTTP header value.
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Represents an invalid request with a custom message.
    #[error("Invalid request: {0}")]
    InvalidRequest(Box<str>),

    /// Client needs to be authenticated to perform the requested action.
    #[error("Client is not authenticated")]
    NotAuthenticated,

    /// General HTTP client error from `reqwest`.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Invalid secret key provided.
    #[error("Invalid secret key")]
    SecretKey,

    /// Error during JSON serialization or deserialization.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),

    /// Error working with system time.
    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    /// UTF-8 decoding error.
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    /// Invalid URL format.
    #[error("Invalid URL: {0}")]
    UrlParseError(Box<str>),
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParseError(e.to_string().into_boxed_str())
    }
}
