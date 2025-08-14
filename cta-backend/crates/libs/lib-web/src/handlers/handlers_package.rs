use crate::error::{Error, Result};
use axum::{extract::State, Json};
use lib_core::{
    ctx::Ctx,
    model::{
        package::{self, LocationForCreate, PackageForCreate},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn api_create_package(
    State(mm): State<ModelManager>,
    Json(payload): Json<CreatePackagePayload>,
) -> Result<Json<Value>> {
    let ctx = Ctx::root_ctx();
    let CreatePackagePayload {
        name,
        description,
        duration_days,
    } = payload;
    let package_c = PackageForCreate {
        name: name.to_string(),
        description: description.to_string(),
        duration_days,
    };
    let id = package::PackgeBmc::create_package(&ctx, &mm, package_c)
        .await
        .map_err(|e| Error::CannotCreatePackage {
            title: format!("Failed to create package: {name}"),
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
pub struct CreatePackagePayload {
    pub name: String,
    pub description: String,
    pub duration_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationPayload {
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}
