use log::{error, info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::city::City;
use crate::schema::city::dsl::*;

#[derive(Deserialize)]
pub struct CityQuery {
    city: Option<String>,
}

/// Fetches all cities from the database
pub async fn get_cities(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<CityQuery>,
) -> Result<Json<Vec<City>>, AppError> {
    info!("GET /cities - fetching cities");

    let mut conn = pool.get()?;

    let mut query_builder = city.select(City::as_select()).into_boxed();

    if let Some(city_val) = query.city {
        info!("GET /cities - filtering by city: {}", city_val);
        let pattern = format!("%{}%", city_val);
        query_builder = query_builder.filter(city_col.ilike(pattern));
    }

    let results = query_builder.load(&mut conn).map_err(|e| {
        error!("Failed to fetch cities: {}", e);
        AppError::Database(e)
    })?;

    if results.is_empty() {
        warn!("GET /cities - found 0 cities");
        return Ok(Json(results));
    }
    info!("GET /cities - found {} cities", results.len());
    Ok(Json(results))
}
