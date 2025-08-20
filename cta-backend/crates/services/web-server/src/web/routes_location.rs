use axum::{
    routing::{get, post},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_location;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route(
            "/location/get/{id}",
            get(handlers_location::api_get_location),
        )
        .route("/location", post(handlers_location::api_create_location))
        .with_state(mm)
}
