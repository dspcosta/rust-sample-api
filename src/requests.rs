use axum::extract::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use diesel::prelude::*;

use crate::DbPool;
use crate::models::Customer;
use crate::schema::customer::dsl::*;

pub async fn get_foo() -> &'static str {
    "you have requested /get foo again\n"
}

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
