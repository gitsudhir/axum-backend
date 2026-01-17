use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct HealthResponse {
    /// Status of the service
    pub status: String,
    /// Timestamp of the health check
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    /// Unique identifier for the user
    pub id: i32,
    /// User's email address
    pub email: String,
    /// User's full name
    pub name: String,
    /// Account creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateUserRequest {
    /// User's email address
    pub email: String,
    /// User's full name
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Wallet {
    /// Unique identifier for the wallet
    pub id: i32,
    /// Associated user ID
    pub user_id: i32,
    /// Current balance
    pub balance: f64,
    /// Currency code (e.g., USD, EUR)
    pub currency: String,
    /// Wallet creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferRequest {
    /// Source wallet ID
    pub from_wallet_id: i32,
    /// Destination wallet ID
    pub to_wallet_id: i32,
    /// Amount to transfer
    pub amount: f64,
    /// Optional idempotency key
    pub idempotency_key: Option<String>,
}

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    pub version: String,
    pub uptime: String,
    pub server_time: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Product {
    /// Unique identifier for the product
    pub id: i32,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price
    pub price: f64,
    /// Product category
    pub category: String,
    /// Product creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateProductRequest {
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product price
    pub price: f64,
    /// Product category
    pub category: String,
}