use log::{error, info, warn};

use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Query;
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::customer::Customer;
use crate::schema::customer::dsl::*;

#[derive(Deserialize)]
pub struct CustomerQuery {
    create_date: Option<String>,
}

/// Fetches all customers from the database
pub async fn get_customers(
    Extension(pool): Extension<DbPool>,
    Query(query): Query<CustomerQuery>,
) -> Result<Json<Vec<Customer>>, AppError> {
    info!("GET /customers - fetching customers");

    let mut conn = pool.get()?;

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
