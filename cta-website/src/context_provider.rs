use leptos::{
    context::Provider,
    prelude::*,
    server::codee::string::{FromToStringCodec, JsonSerdeCodec},
};
use leptos_meta::*;
use leptos_use::storage::use_local_storage;

use crate::types::theme::Themes;

#[derive(Debug, Clone, Default)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Copy, Debug)]
pub struct ConfigProvider {
    pub theme: RwSignal<String>,
    auth_token: RwSignal<Tokens>,
}

impl ConfigProvider {
    pub fn new() -> Self {
        let (stored_theme, _, _) = use_local_storage::<String, JsonSerdeCodec>("theme");
        let (access_token, _, _) = use_local_storage::<String, FromToStringCodec>("access-token");
        let (refresh_token, _, _) = use_local_storage::<String, FromToStringCodec>("refresh-token");

        let theme = stored_theme.get();
        let tokens = Tokens {
            access_token: access_token.get(),
            refresh_token: refresh_token.get(),
        };

        Self {
            theme: RwSignal::new(theme),
            auth_token: RwSignal::new(tokens),
        }
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn logged_in(&self) -> bool {
        !self.auth_token.get().access_token.is_empty()
            && !self.auth_token.get().refresh_token.is_empty()
    }

    pub fn access_token(&self) -> String {
        self.auth_token.get().access_token.clone()
    }

    pub fn set_tokens(&mut self, tokens: Tokens) {
        self.auth_token.set(tokens);
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
                attr:data-theme=move || config_injection.theme.get()
                attr:class="font-arimo"
            />

            // sets the document title
            <Title text="Welcome to CTA Admin" />

            // injects metadata in the <head> of the page
            <Meta charset="UTF-8" />
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

            {children()}
        </Provider>
    }
    .into_any()
}
