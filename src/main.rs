use axum::{
    Router,
    extract::{Extension, Json},
    http::StatusCode,
    routing::{get, post},
};

mod models;
mod requests;
mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

// Create a type alias for our connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize, Debug)]
struct Payload {
    data: String,
}

#[derive(serde::Serialize)]
struct SuccessResponse {
    message: String,
    received_data: String,
}

fn init_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    // Create a pool with 5 connections
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

async fn post_foo(Json(payload): Json<Payload>) -> (StatusCode, Json<SuccessResponse>) {
    log::info!("POST /foo received data: {:?}", payload);
    let message = "Successfully processed the data!".to_string();
    (
        StatusCode::OK,
        Json(SuccessResponse {
            message,
            received_data: payload.data,
        }),
    )
}

#[tokio::main]
async fn main() {
    let pool = init_pool();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/foo", get(requests::get_foo))
        .route("/foo_post", post(post_foo))
        .route("/customers", get(requests::get_customers))
        // Add the pool to the app state so all routes can access it
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
