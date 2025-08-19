

use crate::{error::Error, types::request_types::{LoginPayload, LoginReturn, LogoffPayload, LogoffReturn}, utils::api::request_post};

pub async fn api_login_req(data: LoginPayload) -> Result<LoginReturn, Error> {
    request_post::<LoginPayload, LoginReturn>("/login".into(), data).await
}

pub async fn api_logout_req(data: LogoffPayload) -> Result<LogoffReturn, Error> {
    request_post::<LogoffPayload, LogoffReturn>("/logoff".into(), data).await
}

pub async fn api_get_refresh_token(refresh_token: String) -> Result<LoginReturn, Error> {
    request_post::<String, LoginReturn>("/refresh-token".into(), refresh_token).await
}
