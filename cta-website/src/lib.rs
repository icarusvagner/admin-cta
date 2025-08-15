use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

// Modules
mod components;
mod config;
mod context_provider;
mod error;
mod layouts;
mod pages;
mod types;
mod utils;

// Top-Level pages
use crate::{
    context_provider::ConfigProvider,
    layouts::main_layout::MainLayout,
    pages::{home::Home, login::LoginPage, not_found::NotFound},
};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <ConfigProvider>
            <Router>
                <Routes fallback=|| view! { <NotFound /> }>
                    <ParentRoute path=StaticSegment("/") view=MainLayout>
                        <Route path=StaticSegment("") view=Home />
                    </ParentRoute>
                    <Route path=StaticSegment("login") view=LoginPage />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}
