use std::net::{Ipv4Addr, SocketAddr};
use axum::Router;

mod config;
mod state;
mod models;
mod handlers;
mod routes;
mod api;

use crate::{state::AppState, config::env::Config, api::get_swagger_ui};

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let config = Config::from_env().expect("Failed to load configuration");

    // Build our application with routes
    let app = Router::new()
        .merge(routes::create_routes())
        .merge(get_swagger_ui());

    // Get the port number from the environment, default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Get the port as a string or default to "3000"
        .parse() // Parse the port string into a u16
        .expect("Failed to parse PORT");

    // Create a socket address (IPv4 binding)
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("🚀 Server running on http://0.0.0.0:{}", port);
    println!("📖 API Documentation: http://0.0.0.0:{}/swagger-ui", port);
    println!("📄 OpenAPI JSON: http://0.0.0.0:{}/api-docs/openapi.json", port);

    // Run the app with hyper, listening on the specified address
    axum::serve(listener, app).await.unwrap();
}