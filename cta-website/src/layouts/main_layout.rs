use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{navbar::NavbarMenu, sidebar::SidebarMenu};

#[component]
pub fn MainLayout() -> AnyView {
    let view_margin = RwSignal::new(String::from("ml-64"));

    view! {
        <section class="flex min-h-screen bg-base-200">
            // <!-- Sidebar -->
            <SidebarMenu view_margin />

            // <!-- Main content area -->
            <div class=move || {
                format!(
                    "flex flex-col flex-1 duration-300 transition-all ease-in-out {}",
                    view_margin.get(),
                )
            }>
                // <!-- Navbar -->
                <NavbarMenu />

                // <!-- Scrollable Content -->
                <main class="overflow-y-auto flex-1 p-3 mt-12">
                    <Outlet />
                </main>
            </div>
        </section>
    }
    .into_any()
}
