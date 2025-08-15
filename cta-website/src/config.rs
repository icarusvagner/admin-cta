use std::sync::OnceLock;

use crate::utils::envs::{self, get_env};

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("WebConfig FATAL - WHILE LOADING CONF - cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub DEV_REQ_URL: String,
    pub PROD_REQ_URL: String,
    pub TEST_REQ_URL: String,
}

impl WebConfig {
    fn load_from_env() -> envs::Result<WebConfig> {
        Ok(WebConfig {
            DEV_REQ_URL: get_env("REQUEST_DEV_URL")?,
            PROD_REQ_URL: get_env("REQUEST_PROD_URL")?,
            TEST_REQ_URL: get_env("REQUEST_TEST_URL")?,
        })
    }
}
