use crate::{error::Error, types::request_types::{LoginPayload, LoginReturn}, utils::api::request_post};


pub async fn api_login_req(data: LoginPayload) -> Result<LoginReturn, Error> {
    request_post::<LoginPayload, LoginReturn>("/login".into(), data).await
}
