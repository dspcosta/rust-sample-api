use log::{error, info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::address::{Address, NewAddress};
use crate::schema::address::dsl::*;

#[derive(Deserialize)]
pub struct AddressQuery {
    district: Option<String>,
}

/// Fetches all addresses from the database
pub async fn get_addresses(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<Address>>, AppError> {
    info!("GET /addresses - fetching addresses");

    let mut conn = pool.get()?;

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

    if results.is_empty() {
        warn!("GET /addresses - found 0 addresses");
        return Ok(Json(results));
    }
    info!("GET /addresses - found {} addresses", results.len());
    Ok(Json(results))
}

pub async fn create_address(
    Extension(pool): Extension<DbPool>,
    body: Result<Json<NewAddress>, JsonRejection>,
) -> Result<(StatusCode, Json<Address>), AppError> {
    info!("POST /addresses - creating address");
    let Json(new_address) = body.map_err(|e| AppError::BadRequest(e.body_text()))?;

    let mut conn = pool.get()?;

    let created = diesel::insert_into(address)
        .values(&new_address)
        .get_result::<Address>(&mut conn)
        .map_err(|e| {
            error!("Failed to create address: {}", e);
            AppError::Database(e)
        })?;

    info!("POST /addresses - created address: {}", created.address_id);
    Ok((StatusCode::CREATED, Json(created)))
}
