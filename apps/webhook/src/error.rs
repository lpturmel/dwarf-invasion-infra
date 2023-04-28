use axum::{http::StatusCode, response::IntoResponse};
use ed25519_dalek::SignatureError;
use hex::FromHexError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingSignature,
    MissingTimestamp,
    BadSignature,
    SerializeError(serde_json::Error),
    BadCommand,
    // Add char
    // Roster

    // Del char
    // Roll
    // queue sims
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, String) {
        match self {
            Error::MissingSignature => (StatusCode::BAD_REQUEST, "Missing signature".to_string()),
            Error::MissingTimestamp => (StatusCode::BAD_REQUEST, "Missing timestamp".to_string()),
            Error::BadSignature => (StatusCode::UNAUTHORIZED, "Bad signature".to_string()),
            Error::SerializeError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error serializing body".to_string(),
            ),
            Error::BadCommand => (
                StatusCode::BAD_REQUEST,
                "Bad command, not recognized".to_string(),
            ),
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl From<FromHexError> for Error {
    fn from(_: FromHexError) -> Self {
        Error::BadSignature
    }
}

impl From<SignatureError> for Error {
    fn from(_: SignatureError) -> Self {
        Error::BadSignature
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerializeError(e)
    }
}
