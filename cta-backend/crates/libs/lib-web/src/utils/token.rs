pub use crate::error::ClientError;
pub use crate::error::{Error, Result};
use lib_auth::token::generate_web_token;
use tower_cookies::cookie::SameSite;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};
use uuid::Uuid;

// endregion: --- Modules

pub(crate) const AUTH_TOKEN: &str = "auth-token";

pub(crate) fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
    let token = generate_web_token(user, salt)?;
    let mut now = OffsetDateTime::now_utc();
    now += Duration::weeks(52);

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_path("/"); // <-- critical: works for all routes
    cookie.set_secure(true); // only over HTTPS
    cookie.set_http_only(true); // not accessible from JS
    cookie.set_same_site(SameSite::Lax); // or None if frontend != backend
    cookie.set_max_age(Duration::days(3));

    cookies.add(cookie);

    Ok(())
}

pub(crate) fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
