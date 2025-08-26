use leptos::prelude::*;
use phosphor_leptos::{Icon, SPINNER};

#[component]
pub fn LoadingComponent() -> AnyView {
    view! {
        <div class="flex flex-col gap-5 justify-center items-center p-24 w-full h-full">
            <Icon
                icon=SPINNER
                attr:class="animate-spin duration-300 transition-all h-8 w-8"
            />
        </div>
    }
    .into_any()
}
