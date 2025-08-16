use leptos::prelude::*;
use leptos_router::{components::*, StaticSegment};

use crate::{
    context_provider::ConfigProvider, layouts::main_layout::MainLayout, pages::home::Home,
};

#[component]
pub fn HomeRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    let config_context = ConfigProvider::expect_context();

    view! {
        <ProtectedParentRoute
            path=StaticSegment("/")
            condition=move || Some(config_context.logged_in())
            redirect_path=|| "/login"
            view=MainLayout
        >
            <AppRoutes />
        </ProtectedParentRoute>
    }
    .into_inner()
}

#[component(transparent)]
fn AppRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! { <Route path=StaticSegment("") view=Home /> }.into_inner()
}
