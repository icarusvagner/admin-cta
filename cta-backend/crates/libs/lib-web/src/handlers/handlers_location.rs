use crate::error::{Error, Result};
use axum::{
    extract::{Path, State},
    Json,
};
use lib_core::{
    ctx::Ctx,
    model::{
        package::{self, Location, LocationForCreate},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn api_get_location(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();
    let location = package::PackgeBmc::get_location_by_id::<Location>(&ctx, &mm, id)
        .await
        .map_err(|ex| Error::FailToGetData {
            title: "Failed to get one location".to_string(),
            reason: ex.to_string(),
        })?;

    let body = Json(json!({
        "result" : {
            "success": true,
            "data": location
        }
    }));

    Ok(body)
}

pub async fn api_create_location(
    State(mm): State<ModelManager>,
    Json(payload): Json<CreateLocationPayload>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();
    let CreateLocationPayload {
        name,
        city,
        province,
        category,
        description,
    } = payload;
    let location_c = LocationForCreate {
        name: name.to_string(),
        city: city.to_string(),
        province: province.to_string(),
        category: category.to_string(),
        description: description.to_string(),
    };
    let id = package::PackgeBmc::create_location(&ctx, &mm, location_c)
        .await
        .map_err(|e| Error::CannotCreatePackage {
            title: format!("Failed to create location: {name}"),
            reason: e.to_string().replace("\"", "'"),
        })?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
            "id": id,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationPayload {
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}
