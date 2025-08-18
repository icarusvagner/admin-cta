use crate::error::{Error, Result};
use crate::utils::token::{set_token_cookie, AUTH_TOKEN};
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lib_auth::token::{validate_web_token, Token};
use lib_core::ctx::{AuthUser, Ctx};
use lib_core::model::admin::{AdminBmc, AdminForAuth};
use lib_core::model::ModelManager;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require(ctx: Result<CtxW>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// IMPORTANT: This resolver must never fail, but rather capture the potential Auth error and put in in the
//            request extension as CtxExtResult.
//            This way it won't prevent downstream middleware to be executed, and will still capture the error
//            for the appropriate middleware (.e.g., mw_ctx_require which forces successful auth) or handler
//            to get the appropriate information.
pub async fn mw_ctx_resolver(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(mm: ModelManager, cookies: &Cookies) -> CtxExtResult {
    // -- Get Token String
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    // -- Parse Token
    let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

    // -- Get UserForAuth
    let user: AdminForAuth = AdminBmc::first_by_uname(&Ctx::root_ctx(), &mm, &token.ident)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    // -- Validate Token
    validate_web_token(&token, user.token_salt).map_err(|_| CtxExtError::FailValidate)?;

    // -- Update Token
    set_token_cookie(cookies, &user.uname, user.token_salt)
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // -- Create CtxExtResult
    Ctx::new(user.id)
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region:    --- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    Unauthorized,

    TokenNotInHeader,
    TokenNotInCookie,
    TokenWrongFormat,

    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
pub async fn mw_auth_ctx_require(
    auth_ctx: Result<AuthUserW>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_auth_ctx_require - {auth_ctx:?}", "MIDDLEWARE");

    auth_ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_auth_resolver(
    State(mm): State<ModelManager>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_auth_resolver", "MIDDLEWARE");

    let auth_ext_result = auth_resolver(mm, &req).await;

    if auth_ext_result.is_err()
        && !matches!(auth_ext_result, Err(AuthUserExtError::TokenNotInHeader))
    {
        req.headers_mut().remove(AUTHORIZATION);
    }

    req.extensions_mut().insert(auth_ext_result);

    next.run(req).await
}

async fn auth_resolver(mm: ModelManager, req: &Request<Body>) -> AuthUserExtResult {
    let auth_token = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthUserExtError::TokenNotInHeader)?;

    let auth_str = auth_token
        .to_str()
        .map_err(|_| AuthUserExtError::Unauthorized)?;

    let Some(("Bearer", token)) = auth_str.split_once(' ') else {
        Err(AuthUserExtError::TokenWrongFormat)?
    };

    let token: Token = token
        .parse()
        .map_err(|_| AuthUserExtError::TokenWrongFormat)?;

    let user: AdminForAuth = AdminBmc::first_by_uname(&Ctx::root_ctx(), &mm, &token.ident)
        .await
        .map_err(|ex| AuthUserExtError::ModelAccessError(ex.to_string()))?
        .ok_or(AuthUserExtError::UserNotFound)?;

    validate_web_token(&token, user.token_salt).map_err(|_| AuthUserExtError::FailValidate)?;

    AuthUser::new(user.uname)
        .map(AuthUserW)
        .map_err(|ex| AuthUserExtError::AuthUserCtxCreateFail(ex.to_string()))
}

// region: --- Token extractor

#[derive(Debug, Clone)]
pub struct AuthUserW(pub AuthUser);

impl<S: Send + Sync> FromRequestParts<S> for AuthUserW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - AuthUserW", "EXTRACTOR");

        parts
            .extensions
            .get::<AuthUserExtResult>()
            .ok_or(Error::AuthUserExt(AuthUserExtError::TokenNotInRequestExt))?
            .clone()
            .map_err(Error::AuthUserExt)
    }
}

// endregion: --- Token extractor

// region:    --- AuthUserExt Extractor Result/Error
type AuthUserExtResult = core::result::Result<AuthUserW, AuthUserExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum AuthUserExtError {
    TokenNotInHeader,
    TokenWrongFormat,

    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokens,
    Unauthorized,

    TokenNotInRequestExt,

    AuthUserContextNotInRequestExt,
    AuthUserCtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
