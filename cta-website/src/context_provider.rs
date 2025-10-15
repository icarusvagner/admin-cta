use leptos::{
    context::Provider,
    prelude::*,
    server::codee::string::{FromToStringCodec, JsonSerdeCodec},
};
use leptos_meta::*;
use leptos_use::storage::use_local_storage;

use crate::{
    types::{theme::Themes, user::CurrentUserToken},
    utils::api::set_token,
};

#[derive(Clone, Copy, Debug)]
pub struct ConfigProvider {
    pub theme: RwSignal<String>,
    pub access_token: RwSignal<Option<String>>,
    pub refresh_token: RwSignal<Option<String>>,
}

impl ConfigProvider {
    pub fn new() -> Self {
        let (stored_theme, _, _) = use_local_storage::<String, JsonSerdeCodec>("theme");

        Self {
            theme: RwSignal::new(stored_theme.get()),
            access_token: RwSignal::new(None),
            refresh_token: RwSignal::new(None),
        }
    }

    pub fn login(&self, value: CurrentUserToken) {
        set_token("access_token", Some(value.access_token.clone()));
        set_token("refresh_token", Some(value.refresh_token.clone()));

        self.access_token.set(Some(value.access_token));
        self.refresh_token.set(Some(value.refresh_token));
    }

    pub fn logoff(&self) {
        set_token("access_token", None);
        set_token("refresh_token", None);
        self.access_token.set(None);
        self.refresh_token.set(None);
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn logged_in(&self) -> bool {
        self.access_token.get().is_some() && self.refresh_token.get().is_some()
    }

    pub fn update_theme(&self, theme: Themes) {
        let (_, set_state, _) = use_local_storage::<String, JsonSerdeCodec>("theme");
        self.theme.update(|val| *val = theme.as_str().into());

        set_state.set(self.theme.get());
    }
}

#[component]
pub fn ConfigProvider(children: Children) -> AnyView {
    let config_injection = ConfigProvider::new();

    view! {
        <Provider value=config_injection>
            <Html
                attr:lang="en"
                attr:dir="ltr"
                attr:data-theme=move || {
                    if !config_injection.theme.get().is_empty() {
                        config_injection.theme.get()
                    } else {
                        "light".to_string()
                    }
                }
                attr:class="font-arimo"
            />

            // sets the document title
            <Title text="Welcome to CTA Admin" />

            // injects metadata in the <head> of the page
            <Meta charset="UTF-8" />
            <Meta
                name="viewport"
                content="width=device-width, initial-scale=1.0"
            />

            {children()}
        </Provider>
    }
    .into_any()
}
