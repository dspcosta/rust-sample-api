use axum::Json;
use jsonwebtoken::{EncodingKey, Header, encode};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: usize,
}

pub async fn login(Json(body): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    info!(
        "POST /auth/login - login attempt for user: {}",
        body.username
    );

    let valid_username = std::env::var("API_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let valid_password = std::env::var("API_PASSWORD").unwrap_or_else(|_| "secret".to_string());

    if body.username != valid_username || body.password != valid_password {
        warn!(
            "POST /auth/login - invalid credentials for user: {}",
            body.username
        );
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600) as usize; // Adding 1 hour to the current time

    let claims = Claims {
        sub: body.username.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    info!(
        "POST /auth/login - login successful for user: {}",
        body.username
    );

    let response = LoginResponse {
        token,
        expires_in: exp,
    };
    Ok(Json(response))
}
