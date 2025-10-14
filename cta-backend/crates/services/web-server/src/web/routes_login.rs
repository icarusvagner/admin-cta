use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_login;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/v1/login", post(handlers_login::api_login_handler))
        .route("/api/v1/logoff", post(handlers_login::api_logoff_handler))
        .with_state(mm)
}
