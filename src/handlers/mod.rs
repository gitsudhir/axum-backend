pub mod docs;

use axum::{Json, http::StatusCode, extract::{Path, Query}};
use serde::Deserialize;
use chrono::Utc;

use crate::models::{HealthResponse, User, CreateUserRequest, Wallet, TransferRequest, HomePageTemplate};
use axum::response::Html;
use askama::Template;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
pub async fn health_check() -> Result<Json<HealthResponse>, StatusCode> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}

#[utoipa::path(
    get,
    path = "/users",
    params(
        ("page" = Option<i32>, Query, description = "Page number for pagination"),
        ("limit" = Option<i32>, Query, description = "Number of items per page")
    ),
    responses(
        (status = 200, description = "List of users", body = [User]),
        (status = 400, description = "Invalid parameters")
    )
)]
pub async fn get_users(
    Query(_params): Query<PaginationParams>,
) -> Result<Json<Vec<User>>, StatusCode> {
    // In a real application, this would fetch from a database
    let users = vec![
        User {
            id: 1,
            email: "john@example.com".to_string(),
            name: "John Doe".to_string(),
            created_at: Utc::now(),
        },
        User {
            id: 2,
            email: "jane@example.com".to_string(),
            name: "Jane Smith".to_string(),
            created_at: Utc::now(),
        },
    ];
    
    Ok(Json(users))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid request data")
    )
)]
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // In a real application, this would save to a database
    let user = User {
        id: 1,
        email: payload.email,
        name: payload.name,
        created_at: Utc::now(),
    };
    
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = User),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user_by_id(
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    // In a real application, this would fetch from a database
    let user = User {
        id,
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        created_at: Utc::now(),
    };
    
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/wallets/{user_id}",
    params(
        ("user_id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User's wallets", body = [Wallet]),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user_wallets(
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Wallet>>, StatusCode> {
    // In a real application, this would fetch from a database
    let wallets = vec![
        Wallet {
            id: 1,
            user_id,
            balance: 100.0,
            currency: "USD".to_string(),
            created_at: Utc::now(),
        },
    ];
    
    Ok(Json(wallets))
}

#[utoipa::path(
    post,
    path = "/transfers",
    request_body = TransferRequest,
    responses(
        (status = 200, description = "Transfer completed successfully", body = TransferRequest),
        (status = 400, description = "Invalid transfer request")
    )
)]
pub async fn create_transfer(
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferRequest>, StatusCode> {
    // In a real application, this would process the transfer
    Ok(Json(payload))
}

pub async fn home_page() -> Html<String> {
    let template = HomePageTemplate {
        version: "1.0.0".to_string(),
        uptime: "0 days, 0 hours, 0 minutes".to_string(), // This would be dynamic in a real app
        server_time: chrono::Utc::now().to_rfc3339(),
    };
    
    Html(template.render().unwrap())
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct PaginationParams {
    /// Page number for pagination
    pub page: Option<i32>,
    /// Number of items per page
    pub limit: Option<i32>,
}