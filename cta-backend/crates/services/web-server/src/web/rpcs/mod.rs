pub mod location_rpc;

use rpc_router::{Router, RouterBuilder};

pub fn all_rpc_router() -> RouterBuilder {
    Router::builder().extend(location_rpc::rpc_router_builder())
}
