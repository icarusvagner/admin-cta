use leptos::{context::Provider, prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_meta::*;
use leptos_use::storage::use_local_storage;

use crate::types::theme::Themes;

#[derive(Debug, Clone, Default)]
pub struct Tokens {
    pub auth_token: String,
}

#[derive(Clone, Copy, Debug)]
pub struct ConfigProvider {
    pub theme: RwSignal<String>,
    auth_token: RwSignal<Tokens>,
}

impl ConfigProvider {
    pub fn new() -> Self {
        let (stored_theme, _, _) = use_local_storage::<String, JsonSerdeCodec>("theme");

        let theme = stored_theme.get();

        Self {
            theme: RwSignal::new(theme),
            auth_token: RwSignal::new(Tokens::default()),
        }
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn logged_in(&self) -> bool {
        !self.auth_token.get().auth_token.is_empty()
    }

    pub fn auth_token(&self) -> String {
        self.auth_token.get().auth_token.clone()
    }

    pub fn set_token(&mut self, auth_token: String) {
        let token = Tokens { auth_token };
        self.auth_token.update(|va| *va = token);
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
