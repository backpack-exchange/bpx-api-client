pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    UrlParseError(String),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Invalid secret key")]
    SecretKey,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
