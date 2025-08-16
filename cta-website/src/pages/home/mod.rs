mod panes;

use crate::pages::home::panes::{pane_1::Pane1, pane_2::Pane2, pane_3::Pane3, pane_4::Pane4};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="grid md:grid-cols-2 gap-10 p-5 h-full">
            <Pane1 />
            <Pane2 />
            <Pane3 />
            <Pane4 />
        </section>
    }
}
