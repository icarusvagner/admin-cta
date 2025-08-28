use leptos::{
    context::Provider,
    prelude::*,
    server::codee::string::{FromToStringCodec, JsonSerdeCodec},
};
use leptos_meta::*;
use leptos_use::storage::use_local_storage;

use crate::types::theme::Themes;

#[derive(Clone, Copy, Debug)]
pub struct ConfigProvider {
    pub theme: RwSignal<String>,
    logged_in: RwSignal<bool>,
}

impl ConfigProvider {
    pub fn new() -> Self {
        let (stored_theme, _, _) = use_local_storage::<String, JsonSerdeCodec>("theme");
        let (logged_in, _, _) = use_local_storage::<bool, JsonSerdeCodec>("logged-in");

        Self {
            theme: RwSignal::new(stored_theme.get()),
            logged_in: RwSignal::new(logged_in.get()),
        }
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn is_logged_in(&mut self) {
        let (login, set_login, _) = use_local_storage::<bool, FromToStringCodec>("logged-in");
        set_login.set(true);

        self.logged_in.update(|va| *va = login.get());
    }

    pub fn logout(&mut self) {
        let (login, set_login, reset) = use_local_storage::<bool, FromToStringCodec>("logged-in");
        set_login.set(false);
        reset();

        self.logged_in.update(|va| *va = login.get());
    }

    pub fn logged_in(&self) -> bool {
        self.logged_in.get()
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
