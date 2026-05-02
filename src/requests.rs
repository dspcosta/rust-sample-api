use axum::extract::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use diesel::prelude::*;

use crate::DbPool;
use crate::models::Address;
use crate::models::City;
use crate::models::Customer;
use crate::schema::address::dsl::*;
use crate::schema::city::dsl::*;
use crate::schema::customer::dsl::*;

pub async fn get_customers(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<Customer>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let results = customer
        .limit(1000)
        .select(Customer::as_select())
        .load(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading customers: {}", e),
            )
        })?;

    Ok(Json(results))
}

pub async fn get_addresses(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<Address>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let results = address
        .limit(1000)
        .select(Address::as_select())
        .load(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading addresses: {}", e),
            )
        })?;

    Ok(Json(results))
}

pub async fn get_cities(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<City>>, (StatusCode, String)> {
    let mut conn = pool.get().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get connection: {}", e),
        )
    })?;

    let results = city
        .limit(1000)
        .select(City::as_select())
        .load(&mut conn)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading addresses: {}", e),
            )
        })?;

    Ok(Json(results))
}
