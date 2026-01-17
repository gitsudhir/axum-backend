use axum::{Router, routing::{get, post}};

use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(handlers::home_page))
        .route("/health", get(handlers::health_check))
        .route("/users", get(handlers::get_users).post(handlers::create_user))
        .route("/users/{id}", get(handlers::get_user_by_id))
        .route("/wallets/{user_id}", get(handlers::get_user_wallets))
        .route("/transfers", post(handlers::create_transfer))
}