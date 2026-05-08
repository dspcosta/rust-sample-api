use log::{error, info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::city::{City, NewCity, UpdateCity};
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

pub async fn create_city(
    Extension(pool): Extension<DbPool>,
    body: Result<Json<NewCity>, JsonRejection>,
) -> Result<(StatusCode, Json<City>), AppError> {
    info!("POST /cities - creating city");
    let Json(new_city) = body.map_err(|e| AppError::BadRequest(e.body_text()))?;

    let mut conn = pool.get()?;

    let created = diesel::insert_into(city)
        .values(&new_city)
        .get_result::<City>(&mut conn)
        .map_err(|e| {
            error!("Failed to create city: {}", e);
            AppError::Database(e)
        })?;

    info!("POST /cities - created city: {}", created.city_id);
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn update_city(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
    body: Result<Json<UpdateCity>, JsonRejection>,
) -> Result<(StatusCode, Json<City>), AppError> {
    info!("PUT /cities/{} - updating city", id);
    let Json(update_data) = body.map_err(|e| AppError::BadRequest(e.body_text()))?;

    let mut conn = pool.get()?;

    let updated = diesel::update(city.filter(city_id.eq(id)))
        .set(&update_data)
        .get_result::<City>(&mut conn)
        .map_err(|e| {
            error!("Failed to update city {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("City {} not found", id))
                }
                _ => AppError::Database(e),
            }
        })?;

    info!("PUT /cities/{} - updated successfully", id);
    Ok((StatusCode::OK, Json(updated)))
}
