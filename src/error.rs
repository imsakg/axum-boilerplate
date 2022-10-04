use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    BadRequest(&'static str),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Internal,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::LoginFail => (
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "Invalid credentials",
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "Authentication required",
            ),
            Self::Forbidden => (StatusCode::FORBIDDEN, "forbidden", "Action not permitted"),
            Self::TicketDeleteFailIdNotFound { .. } | Self::NotFound => {
                (StatusCode::NOT_FOUND, "not_found", "Resource not found")
            }
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            Self::Conflict => (StatusCode::CONFLICT, "conflict", "Resource already exists"),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "An internal error occurred",
            ),
        };
        (
            status,
            Json(json!({"error": {"code": code, "message": message}})),
        )
            .into_response()
    }
}
