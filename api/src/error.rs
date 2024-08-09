use std::fmt;

use axum::{response::IntoResponse, Json};
use base64::DecodeError;
use gluer::metadata;
use hyper::StatusCode;
use serde::Serialize;
use tracing::error;

/// The api compatible error type.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[metadata]
#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Error {
    /// The user provided arguments are malformed
    Arguments,
    /// A file could not be found or opened
    FileOpen,
    /// A file with that name already exists
    AlreadyExists,
    /// An uploaded file has an invalid type
    InvalidFileType,
    /// Could not connect to server
    Network,
    /// Invalid file format
    InvalidFormat,
    /// No matching results
    NothingFound,
    /// Conversion error, decoding, ...
    Conversion,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        error!("convert::Infallible: {e:?}");
        Self::Arguments
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        use std::io::ErrorKind;

        error!("File Error: {e}");
        match e.kind() {
            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::AddrInUse
            | ErrorKind::AddrNotAvailable => Self::Network,
            _ => Self::FileOpen,
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        error!("Invalid JSON Format: {e:?}");
        Self::InvalidFormat
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        error!("Conversion Failed: {e:?}");
        Self::Conversion
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::Arguments
            | Error::InvalidFormat
            | Error::InvalidFileType
            | Error::AlreadyExists
            | Error::Conversion => StatusCode::BAD_REQUEST,
            Error::FileOpen | Error::NothingFound => StatusCode::NOT_FOUND,
            Error::Network => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status, Json(self)).into_response()
    }
}

/// Result type using the api error.
#[metadata]
pub type Result<T> = std::result::Result<T, Error>;
