use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};

// Modules
mod components;
mod config;
mod context_provider;
mod error;
mod home_routes;
mod layouts;
mod pages;
mod server;
mod types;
mod utils;

use config::web_config;

// Top-Level pages
use crate::{
    context_provider::ConfigProvider,
    home_routes::HomeRoutes,
    pages::{login::LoginPage, not_found::NotFound},
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
                    <HomeRoutes />
                    <RouteLogin />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}

#[component]
fn RouteLogin() -> impl leptos_router::MatchNestedRoutes + Clone {
    let config_context = ConfigProvider::expect_context();

    view! {
        <ProtectedRoute
            condition=move || Some(!config_context.logged_in())
            redirect_path=move || "/"
            path=StaticSegment("login")
            view=LoginPage
        />
    }
    .into_inner()
}
