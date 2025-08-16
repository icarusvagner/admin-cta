use crate::error::{Error, Result};
use crate::utils::token;
use axum::extract::State;
use axum::Json;
use lib_auth::pass::{self, ContentToHash, SchemeStatus};
use lib_auth::token::generate_web_token;
use lib_core::ctx::Ctx;
use lib_core::model::admin::{AdminBmc, AdminForLogin};
use lib_core::model::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

// region:    --- Login
pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user: AdminForLogin = AdminBmc::first_by_uname(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;

    // -- Validate the password.
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    let scheme_status = pass::validate_pwd(
        ContentToHash {
            salt: user.pwd_salt,
            content: pwd_clear.clone(),
        },
        pwd,
    )
    .await
    .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    // -- Update password scheme if needed
    if let SchemeStatus::Outdated = scheme_status {
        debug!("pwd encrypt scheme outdated, upgrading.");
        AdminBmc::update_pass(&root_ctx, &mm, user.id, &pwd_clear).await?;
    }

    // -- Set web token.
    token::set_token_cookie(&cookies, &user.uname.clone(), user.token_salt)?;

    let access_token = generate_web_token(&user.uname.clone(), user.token_salt)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    pwd: String,
}
// endregion: --- Login

// region:    --- Logoff
pub async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        token::remove_token_cookie(&cookies)?;
    }

    // Create the success body.
    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct LogoffPayload {
    logoff: bool,
}
// endregion: --- Logoff
