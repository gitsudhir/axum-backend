use axum::Json;
use utoipa::{OpenApi, Modify};
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::get_users,
        handlers::create_user,
        handlers::get_user_by_id,
        handlers::get_user_wallets,
        handlers::create_transfer,
        handlers::get_products,
        handlers::create_product,
        handlers::get_product_by_id,
    ),
    components(
        schemas(
            crate::models::HealthResponse,
            crate::models::User,
            crate::models::CreateUserRequest,
            crate::models::Wallet,
            crate::models::TransferRequest,
            crate::models::Product,
            crate::models::CreateProductRequest,
            handlers::PaginationParams,
        )
    ),
    modifiers(&SecurityAddon),
    info(title = "Axum Backend API", version = "1.0.0")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, _openapi: &mut utoipa::openapi::OpenApi) {
        // Add security schemes here if needed
    }
}

pub fn get_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}

pub async fn openapi_handler() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}