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

    /// Response body could not be deserialized into the expected type.
    ///
    /// Carries the full response body so callers can inspect what the API
    /// actually returned. `Display` shows a truncated preview of the body.
    #[error(
        "Failed to deserialize API response: {source}; body: {}",
        body_preview(body)
    )]
    Deserialize {
        #[source]
        source: serde_json::Error,
        body: Box<str>,
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

/// Maximum number of bytes of a response body included in error messages.
const BODY_PREVIEW_LEN: usize = 2000;

/// Truncates `body` to at most [`BODY_PREVIEW_LEN`] bytes on a char boundary.
fn body_preview(body: &str) -> String {
    if body.len() <= BODY_PREVIEW_LEN {
        return body.to_string();
    }
    let mut end = BODY_PREVIEW_LEN;
    while !body.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...[truncated, {} bytes total]", &body[..end], body.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_preview_does_not_split_multibyte_chars() {
        // 'é' is 2 bytes; 1999 ASCII bytes + 'é' puts byte 2000 mid-character.
        let body = format!("{}é{}", "a".repeat(BODY_PREVIEW_LEN - 1), "b".repeat(100));
        let preview = body_preview(&body);
        assert!(preview.starts_with(&"a".repeat(BODY_PREVIEW_LEN - 1)));
        assert!(preview.contains("truncated"));
    }

    #[test]
    fn body_preview_passes_short_bodies_through() {
        assert_eq!(body_preview("{}"), "{}");
    }
}
