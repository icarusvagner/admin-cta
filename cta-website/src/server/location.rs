
use serde_json::{json, Value};

use crate::{error::Error, types::{location::{CountLocationReturn, ListLocationReturn, LocationReturn}, request_types::CreateLocationPayload}, utils::api::request_post};

pub async fn api_create_location(data: CreateLocationPayload) -> Result<Value, Error> {
    let rpc_data = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "create_location",
        "params": {
            "data": data
        }
    });

    request_post::<Value, Value>("/rpc".into(), rpc_data).await
}

pub async fn api_get_location_by_id(id: i64) -> Result<LocationReturn, Error> {
    let rpc_data = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "get_location",
        "params": {
            "id": id
        }
    });

    request_post::<Value, LocationReturn>("/rpc".into(), rpc_data).await
}

pub async fn api_get_locations() -> Result<ListLocationReturn, Error> {
    let rpc_data = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "list_locations",
        "params": { }
    });

    request_post::<Value, ListLocationReturn>("/rpc".into(), rpc_data).await
}

pub async fn api_count_locations() -> Result<CountLocationReturn, Error> {
    let rpc_data = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "count_location",
        "params": { }
    });

    request_post::<Value, CountLocationReturn>("/rpc".into(), rpc_data).await
}

pub async fn api_remove_location(id: i64) -> Result<Value, Error> {
    let rpc_data = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "delete_location",
        "params": {
            "id": id
        }
    });

    request_post::<Value, Value>("/rpc".into(), rpc_data).await
}
