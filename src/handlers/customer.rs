use axum::{routing::get, routing::put};
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
use crate::models::customer::{Customer, NewCustomer, UpdateCustomer};
use crate::schema::customer::dsl::*;

#[derive(Deserialize)]
pub struct CustomerQuery {
    create_date: Option<String>,
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/customers", get(get_customers).post(create_customer))
        .route(
            "/customers/{id}",
            put(update_customer).delete(delete_customer),
        )
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

pub async fn create_customer(
    Extension(pool): Extension<DbPool>,
    body: Result<Json<NewCustomer>, JsonRejection>,
) -> Result<(StatusCode, Json<Customer>), AppError> {
    info!("POST /customers - creating customer");
    let Json(new_customer) = body.map_err(|e| AppError::BadRequest(e.body_text()))?;

    let mut conn = pool.get()?;

    let created = diesel::insert_into(customer)
        .values(&new_customer)
        .get_result::<Customer>(&mut conn)
        .map_err(|e| {
            error!("Failed to create customer: {}", e);
            AppError::Database(e)
        })?;

    info!(
        "POST /customers - created customer: {}",
        created.customer_id
    );
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn update_customer(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
    body: Result<Json<UpdateCustomer>, JsonRejection>,
) -> Result<(StatusCode, Json<Customer>), AppError> {
    info!("PUT /customers/{} - updating customer", id);
    let Json(update_data) = body.map_err(|e| AppError::BadRequest(e.body_text()))?;

    let mut conn = pool.get()?;

    let updated = diesel::update(customer.filter(customer_id.eq(id)))
        .set(&update_data)
        .get_result::<Customer>(&mut conn)
        .map_err(|e| {
            error!("Failed to update customer {}: {}", id, e);
            match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Customer {} not found", id))
                }
                _ => AppError::Database(e),
            }
        })?;

    info!("PUT /customers/{} - updated successfully", id);
    Ok((StatusCode::OK, Json(updated)))
}

pub async fn delete_customer(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let mut conn = pool.get()?;

    diesel::delete(customer.filter(customer_id.eq(id)))
        .execute(&mut conn)
        .map_err(|e| {
            error!("Failed to delete customer {}: {}", id, e);
            AppError::Database(e)
        })?;

    info!("DELETE /customers/{} - deleted successfully", id);
    Ok(StatusCode::OK)
}
