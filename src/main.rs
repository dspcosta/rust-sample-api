use std::time::Duration;

use axum::{Router, extract::Extension, routing::get, routing::put};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    let pool =
        db::init_pool(10, Some(Duration::from_secs(600))).expect("Failed to initialize DB pool.");

    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello from a demo API built with Rust and Axum!" }),
        )
        .route(
            "/customers",
            get(handlers::customer::get_customers).post(handlers::customer::create_customer),
        )
        .route("/customers/{id}", put(handlers::customer::update_customer))
        .route(
            "/addresses",
            get(handlers::address::get_addresses).post(handlers::address::create_address),
        )
        .route("/addresses/{id}", put(handlers::address::update_address))
        .route(
            "/cities",
            get(handlers::city::get_cities).post(handlers::city::create_city),
        )
        .route("/cities/{id}", put(handlers::city::update_city))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(listener, app).await.expect("Server crashed");
}
