use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;
use std::time::Duration;

use crate::errors::AppError;

// Create a type alias for connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes the database connection pool from DATABASE_URL env var
pub fn init_pool(pool_size: u32, max_lifetime: Option<Duration>) -> Result<DbPool, AppError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|e| {
        AppError::InternalServerError(format!("DATABASE_URL must be set {}", e.to_string()))
    })?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(pool_size)
        .max_lifetime(max_lifetime)
        .build(manager)
        .map_err(AppError::PoolError)
}
