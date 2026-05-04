use log::{info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use diesel::prelude::*;
use serde::Deserialize;

use crate::models::Address;
use crate::models::City;
use crate::models::Customer;
use crate::schema::address::dsl::*;
use crate::schema::city::dsl::*;
use crate::schema::customer::dsl::*;
use crate::DbPool;

#[derive(Deserialize)]
pub struct CustomerQuery {
    create_date: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

/// Fetches all customers from the database
pub async fn get_customers(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<CustomerQuery>,
) -> Result<Json<Vec<Customer>>, (StatusCode, Json<ErrorMessage>)> {
    info!("GET /customers - fetching customers");

    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Failed to get connection: {}", e),
            }),
        )
    })?;

    let mut query_builder = customer.select(Customer::as_select()).into_boxed();

    if let Some(date_str) = query.create_date {
        let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorMessage {
                    message: format!("Invalid date format: {}", e),
                }),
            )
        })?;

        info!("GET /customers - query received: {}", date);
        query_builder = query_builder.filter(create_date.eq(date));
    }

    let results = query_builder.load(&mut conn).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Error loading customers: {}", e),
            }),
        )
    })?;

    if results.len() == 0 {
        warn!("GET /customers - found 0 customers");
        return Ok(Json(results));
    }
    info!("GET /customers - found {} customers", results.len());
    Ok(Json(results))
}

/// Fetches all addresses from the database
pub async fn get_addresses(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<Address>>, (StatusCode, Json<ErrorMessage>)> {
    info!("GET /addresses - fetching addresses");

    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Failed to get connection: {}", e),
            }),
        )
    })?;

    let results = address
        .select(Address::as_select())
        .load(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    message: format!("Error loading addresses: {}", e),
                }),
            )
        })?;

    if results.len() == 0 {
        warn!("GET /addresses - found 0 addresses");
        return Ok(Json(results));
    }
    info!("GET /addresses - found {} addresses", results.len());
    Ok(Json(results))
}

/// Fetches all cities from the database
pub async fn get_cities(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<City>>, (StatusCode, Json<ErrorMessage>)> {
    info!("GET /cities - fetching cities");

    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage {
                message: format!("Failed to get connection: {}", e),
            }),
        )
    })?;

    let results = city
        .select(City::as_select())
        .load(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    message: format!("Error loading cities: {}", e),
                }),
            )
        })?;

    if results.len() == 0 {
        warn!("GET /cities - found 0 cities");
        return Ok(Json(results));
    }
    info!("GET /cities - found {} cities", results.len());
    Ok(Json(results))
}
