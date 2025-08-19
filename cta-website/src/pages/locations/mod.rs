use leptos::{either::Either, prelude::*};
use leptos_meta::Title;

use crate::pages::locations::{create::LocationCreate, list::LocationList};

mod create;
mod list;

#[component]
pub fn LocationPage() -> AnyView {
    let active_tab = RwSignal::new("list");

    view! {
        <Title text="Location" />

        <h1 class="mb-5 text-4xl font-bold">"Locations"</h1>

        <div class="grid">
            <div role="tablist" class="tabs tabs-lift tabs-lg">
                <a
                    role="tab"
                    class=move || {
                        format!(
                            "tab {}",
                            if active_tab.get().eq("list") { "tab-active" } else { "" },
                        )
                    }
                    on:click=move |_| active_tab.set("list")
                >
                    "List"
                </a>
                <a
                    role="tab"
                    class=move || {
                        format!(
                            "tab {}",
                            if active_tab.get().eq("create") {
                                "tab-active"
                            } else {
                                ""
                            },
                        )
                    }
                    on:click=move |_| active_tab.set("create")
                >
                    "New Location"
                </a>
            </div>
            <div class="h-full bg-base-100">
                {move || {
                    if active_tab.get().eq("list") {
                        Either::Left(view! { <LocationList /> })
                    } else {
                        Either::Right(view! { <LocationCreate /> })
                    }
                }}
            </div>
        </div>
    }
    .into_any()
}
