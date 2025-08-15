use leptos::prelude::*;
use leptos_router::components::A;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col items-center justify-center w-xl mx-auto">
            <img src="/images/monster-404.svg" alt="Monter 404" class="w-full object-cover" />
            <A href="/" attr:class="link link-hover link-primary">
                "Go back home"
            </A>
        </section>
    }
}
