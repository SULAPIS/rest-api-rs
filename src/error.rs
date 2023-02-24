use aws_sdk_dynamodb::types::SdkError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[allow(dead_code)]
pub enum AppError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError,
    UserDoeNotExist,
    UserAlreadyExists,
    SdkError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occoured".to_string(),
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token".to_string()),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credential".to_string()),
            Self::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create token".to_string(),
            ),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, "wrong credentials".to_string()),
            Self::UserDoeNotExist => (StatusCode::UNAUTHORIZED, "user does not exists".to_string()),
            Self::UserAlreadyExists => (StatusCode::BAD_REQUEST, "user already exists".to_string()),
            Self::SdkError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

impl<E> From<&SdkError<E>> for AppError {
    fn from(e: &SdkError<E>) -> Self {
        Self::SdkError(e.to_string())
    }
}
