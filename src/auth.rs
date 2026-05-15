use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{DecodingKey, Validation, decode};
use log::warn;

use crate::errors::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub async fn log_rate_limit(req: Request, next: Next) -> Response {
    let uri = req.uri().to_string();
    let response = next.run(req).await;
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        warn!("Rate limit exceeded: {}", uri);
    }
    response
}

pub async fn auth_middleware(req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    match token {
        Some(token) => {
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(_) => next.run(req).await,
                Err(_) => {
                    warn!("Invalid token provided.");
                    AppError::Unauthorized("Invalid token".to_string()).into_response()
                }
            }
        }
        None => AppError::Unauthorized("Missing Authorization header".to_string()).into_response(),
    }
}
