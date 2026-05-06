use axum::Json;
use axum::response::{IntoResponse, Response};
use diesel::r2d2;

#[derive(serde::Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    Database(diesel::result::Error),
    InternalServerError(String),
    BadRequest(String),
    PoolError(r2d2::PoolError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::InternalServerError(i) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, i),
            AppError::BadRequest(b) => (axum::http::StatusCode::BAD_REQUEST, b.to_string()),
            AppError::PoolError(p) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, p.to_string())
            }
        };
        (status, Json(ErrorMessage { message })).into_response()
    }
}
