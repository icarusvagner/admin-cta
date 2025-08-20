
use crate::{error::Error, types::request_types::{CreateLocationPayload, LocationReturn, WithIdReturn}, utils::api::{request_get, request_post}};

pub async fn api_create_location(data: CreateLocationPayload) -> Result<WithIdReturn, Error> {
    request_post::<CreateLocationPayload, WithIdReturn>("/location".into(), data).await
}

pub async fn api_get_location_by_id(data: i64) -> Result<LocationReturn, Error> {
    request_get::<LocationReturn>(format!("/location/get/{data}")).await
}
