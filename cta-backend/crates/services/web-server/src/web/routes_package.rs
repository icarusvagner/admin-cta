use axum::{routing::post, Router};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_package;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/package/create/package",
            post(handlers_package::api_create_package),
        )
        .with_state(mm)
}
