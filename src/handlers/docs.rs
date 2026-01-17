use axum::Json;
use utoipa::OpenApi;

use crate::api::ApiDoc;

pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}