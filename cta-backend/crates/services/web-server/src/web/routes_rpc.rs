use axum::{routing::post, Router};
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_rpc;

use crate::web::rpcs::all_rpc_router;

/// Build the Axum router for '/api/rpc'
/// Note: This will build the `rpc-router::Router` that will be used by th
///       rpc_axum_handler
pub fn routes(mm: ModelManager) -> Router {
    // Build the combinded RPC Router (from `rpc-router` crate)
    let rpc_router = all_rpc_router().append_resource(mm).build();

    // Build the Axum handler for `/rpc`
    Router::new()
        .route("/rpc", post(handlers_rpc::rpc_axum_handler))
        .with_state(rpc_router)
}
