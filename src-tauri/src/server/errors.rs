use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    code: i32,
    message: String,
}

impl ApiError {
    pub fn not_found() -> Self {
        ApiError {
            code: 2,
            message: "Not found".to_owned(),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Error] code: {}, message: {}", self.code, self.message)
    }
}

impl Error for ApiError {}

impl From<sqlx::error::Error> for ApiError {
    fn from(_: sqlx::error::Error) -> Self {
        ApiError {
            code: 1,
            message: "Database error".to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
