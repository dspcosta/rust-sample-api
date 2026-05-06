use log::{error, info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use diesel::prelude::*;
use serde::Deserialize;

use crate::DbPool;
use crate::errors::AppError;
use crate::models::Address;
use crate::models::City;
use crate::models::Customer;
use crate::schema::address::dsl::*;
use crate::schema::city::dsl::*;
use crate::schema::customer::dsl::*;

#[derive(Deserialize)]
pub struct CustomerQuery {
    create_date: Option<String>,
}

#[derive(Deserialize)]
pub struct AddressQuery {
    district: Option<String>,
}

/// Fetches all customers from the database
pub async fn get_customers(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<CustomerQuery>,
) -> Result<Json<Vec<Customer>>, AppError> {
    info!("GET /customers - fetching customers");

    let mut conn = pool.get().map_err(AppError::PoolError)?;

    let mut query_builder = customer.select(Customer::as_select()).into_boxed();

    if let Some(date_str) = query.create_date {
        let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
            warn!("Invalid date format: {}", e);
            AppError::BadRequest(format!("Invalid date format: {}", e))
        })?;

        info!("GET /customers - query received: {}", date);
        query_builder = query_builder.filter(create_date.eq(date));
    }

    let results = query_builder.load(&mut conn).map_err(|e| {
        error!("Failed to fetch customers: {}", e);
        AppError::Database(e)
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
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<Address>>, AppError> {
    info!("GET /addresses - fetching addresses");

    let mut conn = pool.get().map_err(AppError::PoolError)?;

    let mut query_builder = address.select(Address::as_select()).into_boxed();

    if let Some(district_val) = query.district {
        info!("GET /addresses - filtering by district: {}", district_val);
        let pattern = format!("%{}%", district_val);
        query_builder = query_builder.filter(district.ilike(pattern));
    }

    let results = query_builder.load(&mut conn).map_err(|e| {
        error!("Failed to fetch addresses: {}", e);
        AppError::Database(e)
    })?;

    if results.len() == 0 {
        warn!("GET /addresses - found 0 addresses");
        return Ok(Json(results));
    }
    info!("GET /addresses - found {} addresses", results.len());
    Ok(Json(results))
}

/// Fetches all cities from the database
pub async fn get_cities(Extension(pool): Extension<DbPool>) -> Result<Json<Vec<City>>, AppError> {
    info!("GET /cities - fetching cities");

    let mut conn = pool.get().map_err(AppError::PoolError)?;

    let results = city
        .select(City::as_select())
        .load(&mut conn)
        .map_err(|e| {
            error!("Failed to fetch cities: {}", e);
            AppError::Database(e)
        })?;

    if results.len() == 0 {
        warn!("GET /cities - found 0 cities");
        return Ok(Json(results));
    }
    info!("GET /cities - found {} cities", results.len());
    Ok(Json(results))
}
