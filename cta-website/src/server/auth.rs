

use crate::{error::Error, types::request_types::{LoginPayload, LoginReturn}, utils::api::request_post};

pub async fn api_login_req(data: LoginPayload) -> Result<LoginReturn, Error> {
    request_post::<LoginPayload, LoginReturn>("/login".into(), data).await
}

pub async fn api_get_refresh_token(refresh_token: String) -> Result<LoginReturn, Error> {
    request_post::<String, LoginReturn>("/refresh-token".into(), refresh_token).await
}
