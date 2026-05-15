use axum::{
    Router,
    extract::Extension,
    middleware,
    routing::{get, post},
};
use log::info;
use std::{net::SocketAddr, time::Duration};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

mod auth;
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

    let governor_config_login = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(5)
        .finish()
        .unwrap();

    let governor_config_protected = GovernorConfigBuilder::default()
        .per_second(100)
        .burst_size(1000)
        .finish()
        .unwrap();

    let governor_limiter_login = governor_config_login.limiter().clone();
    let governor_limiter_protected = governor_config_protected.limiter().clone();
    let interval = Duration::from_secs(60);

    // a separate background task to clean up
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            info!(
                "rate limiting storage size: {}",
                governor_limiter_login.len()
            );
            governor_limiter_login.retain_recent();

            info!(
                "rate limiting storage size for protected routes: {}",
                governor_limiter_protected.len()
            );
            governor_limiter_protected.retain_recent();
        }
    });

    let pool =
        db::init_pool(10, Some(Duration::from_secs(600))).expect("Failed to initialize DB pool.");

    let protected = Router::new()
        .merge(handlers::customer::router())
        .merge(handlers::address::router())
        .merge(handlers::city::router())
        .layer(middleware::from_fn(auth::auth_middleware))
        .layer(GovernorLayer::new(governor_config_protected));

    let login = Router::new()
        .route("/auth/login", post(handlers::auth::login))
        .layer(GovernorLayer::new(governor_config_login));

    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello from a demo API built with Rust and Axum!" }),
        )
        .merge(protected)
        .merge(login)
        .layer(Extension(pool))
        .layer(middleware::from_fn(auth::log_rate_limit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server crashed");
}
