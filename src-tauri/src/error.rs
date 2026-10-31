//! Error type for the launcher.
//!
//! Variants carry a stable, machine-readable code (the `Display` string).
//! The frontend matches on these codes to show a translated message, so the
//! codes here must stay in sync with the `error.*` keys in the locale files.

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network")]
    Network,

    #[error("auth_cancelled")]
    AuthCancelled,

    #[error("auth_no_xbox")]
    AuthNoXbox,

    #[error("auth_child")]
    AuthChild,

    #[error("auth_no_game")]
    AuthNoGame,

    #[error("auth_region")]
    AuthRegion,

    /// Generic auth failure with a detail string (not individually translated).
    #[error("auth: {0}")]
    Auth(String),

    #[error("not_found: {0}")]
    NotFound(String),

    /// Anything else; surfaced via the `error.unknown` key with this detail.
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn other(msg: impl Into<String>) -> Self {
        Error::Other(msg.into())
    }
}

// Commands return these to the webview, so they must serialize. We serialize
// as the Display string (the stable code / message).
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        // Decode/body errors are rare here; treat everything as a network issue
        // but keep the detail for logs.
        if e.is_decode() {
            Error::Other(format!("decode: {e}"))
        } else {
            Error::Network
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Other(format!("io: {e}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Other(format!("json: {e}"))
    }
}

impl From<keyring::Error> for Error {
    fn from(e: keyring::Error) -> Self {
        match e {
            keyring::Error::NoEntry => Error::NotFound("keyring entry".into()),
            other => Error::Other(format!("keyring: {other}")),
        }
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Self {
        Error::Other(format!("zip: {e}"))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
