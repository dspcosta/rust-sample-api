use axum::{Router, extract::Extension, routing::get};

mod models;
mod requests;
mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

// Create a type alias for connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn init_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create a pool with 10 connections (default)
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[tokio::main]
async fn main() {
    let pool = init_pool();

    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello to a simple Rust web server!\n" }),
        )
        .route("/customers", get(requests::get_customers))
        .route("/addresses", get(requests::get_addresses))
        .route("/cities", get(requests::get_cities))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
