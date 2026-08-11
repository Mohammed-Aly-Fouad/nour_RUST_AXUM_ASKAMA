mod api;
mod web;
mod common;
mod domain;
mod error;
mod startup;
mod state;
use state::AppState;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the global logging subscriber (safe against multi-threaded test panics)
    startup::logging();
    tracing::info!("Starting application boot sequence...");

    // 2. Establish connection to the PostgreSQL database pool
    let pool = startup::database_connection().await;
    tracing::info!("Database connection pool established successfully.");

    // 3. Wrap the database pool inside the shared application state
    let state = AppState { pool };
    tracing::info!("Application state initialized.");

    // 4. Configure CORS layers to allow cross-origin requests
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    tracing::debug!("CORS middleware layer configured.");

   // 5. Build the Axum router with state and middleware attached
let app = Router::new()
    .nest("/api", crate::api::router())
    .nest("/web", crate::web::router())
    .nest_service("/assets", ServeDir::new("assets"))
    .layer(cors)
    .with_state(state);
    // 6. Resolve server address from environment variable or fall back to 127.0.0.1:3000
    let addr_str = dotenvy::var("SERVER_ADDRESS").unwrap_or_else(|_| {
        tracing::warn!("SERVER_ADDRESS not found in environment, falling back to 127.0.0.1:3000");
        "127.0.0.1:3000".to_string()
    });

    let addr: SocketAddr = addr_str
        .parse()
        .expect("Failed to parse SERVER_ADDRESS into a valid socket address");

    // 7. Bind the TCP listener and start the Axum web server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Stationery Server successfully running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}