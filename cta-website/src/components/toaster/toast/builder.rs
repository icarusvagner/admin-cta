use std::fmt::Display;

use leptos::prelude::RwSignal;

use crate::components::toaster::toast::data::{ToastData, ToastId, ToastLevel, ToastPosition};

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
            position: ToastPosition::TopRight,
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
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn build(self, id: ToastId) -> ToastData {
        ToastData {
            id,
            level: self.level,
            expiry: self.expiry,
            position: self.position,
            clear_signal: RwSignal::new(false),
        }
    }
}
