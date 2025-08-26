mod toast;

use std::fmt::Display;

pub struct ToastBuilder {
    message: String,

    level: ToastLevel,
    expiry: Option<u32>,
    position: ToastPosition,
}

impl ToastBuilder {
    #[must_use]
    pub fn new<T>(message: T) -> Self
    where
        T: Display,
    {
        ToastBuilder {
            message: message.to_string(),
            level: ToastLevel::Info,
            expiry: Some(2_500),
            position: ToastPosition::Top,
        }
    }

    #[must_use]
    pub fn with_level(mut self, level: ToastLevel) -> Self {
        self.level = level;
        self
    }

    #[must_use]
    pub fn with_expiry(mut self, expiry: Option<u32>) -> Self {
        self.expiry = expiry;
        self
    }

    #[must_use]
    pub fn build() ->  {
        
    }
}
