
use crate::{error::Error, types::request_types::{CreateLocationPayload, WithIdReturn}, utils::api::request_post};

pub async fn api_create_location(data: CreateLocationPayload) -> Result<WithIdReturn, Error> {
    request_post::<CreateLocationPayload, WithIdReturn>("/package/create/location".into(), data).await
}
