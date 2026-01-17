pub mod config;
pub mod state;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod api;

#[cfg(test)]
mod tests {
    use utoipa::OpenApi;

    #[test]
    fn test_api_documentation_generation() {
        use crate::api::ApiDoc;
        
        // Test that the API documentation can be generated without errors
        let api_doc = ApiDoc::openapi();
        
        // Verify that the API doc contains expected paths
        assert!(api_doc.paths.paths.contains_key("/health"));
        assert!(api_doc.paths.paths.contains_key("/users"));
        assert!(api_doc.paths.paths.contains_key("/users/{id}"));
        assert!(api_doc.paths.paths.contains_key("/wallets/{user_id}"));
        assert!(api_doc.paths.paths.contains_key("/transfers"));
        
        // Verify that the API doc contains expected components (schemas)
        assert!(api_doc.components.is_some());
        let components = api_doc.components.as_ref().unwrap();
        assert!(components.schemas.contains_key("HealthResponse"));
        assert!(components.schemas.contains_key("User"));
        assert!(components.schemas.contains_key("CreateUserRequest"));
        assert!(components.schemas.contains_key("Wallet"));
        assert!(components.schemas.contains_key("TransferRequest"));
        
        println!("✅ API documentation generated successfully with {} paths", api_doc.paths.paths.len());
        println!("✅ API documentation contains {} schemas", components.schemas.len());
    }
}