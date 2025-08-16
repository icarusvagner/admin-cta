use leptos::prelude::*;

#[component]
pub fn Pane2() -> AnyView {
    view! {
        <div class="flex flex-col gap-5 rounded border-accent bg-base-300 p-5 items-center justify-center">
            <div class="mx-auto w-full mm:w-sm">
                <h1 class="text-4xl font-black tracking-wide">Promo Packages</h1>
                <div class="text-8xl tabular-nums">01</div>
            </div>
        </div>
    }.into_any()
}
