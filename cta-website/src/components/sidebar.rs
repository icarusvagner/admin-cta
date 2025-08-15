use std::fmt::Display;

use leptos::{either::Either, prelude::*};
use phosphor_leptos::{
    Icon, IconData, CARET_DOUBLE_LEFT, CARET_DOUBLE_RIGHT, CLIPBOARD_TEXT, HOUSE, MAP_PIN_AREA,
    PACKAGE, TREASURE_CHEST,
};

#[derive(Debug, Clone, Default)]
struct ToggleState {
    class: String,
    state: bool,
}

#[component]
pub fn SidebarMenu(#[prop(into)] view_margin: RwSignal<String>) -> AnyView {
    let menu_state = RwSignal::new(ToggleState {
        class: "w-64".into(),
        state: true,
    });

    view! {
        <aside class=move || {
            format!(
                "fixed top-0 left-0 h-screen bg-base-300 duration-300 transition-all ease-in-out {}",
                menu_state.get().class,
            )
        }>
            <div class="relative w-full flex flex-col gap-4 h-full justify-center px-3">
                <div class="absolute -right-5 top-3">
                    <button
                        class="btn btn-ghost btn-circle"
                        on:click=move |_| {
                            let mut curr = menu_state.get();
                            if curr.state {
                                view_margin.update(|val| *val = String::from("ml-24"));
                                curr.class = String::from("w-24");
                                curr.state = false;
                            } else {
                                view_margin.update(|val| *val = String::from("ml-64"));
                                curr.class = String::from("w-64");
                                curr.state = true;
                            }
                            menu_state.set(curr);
                        }
                    >
                        {move || {
                            if !menu_state.get().state {
                                Either::Left(
                                    view! { <Icon icon=CARET_DOUBLE_RIGHT attr:class="h-6 w-6" /> },
                                )
                            } else {
                                Either::Right(
                                    view! { <Icon icon=CARET_DOUBLE_LEFT attr:class="h-6 w-6" /> },
                                )
                            }
                        }}
                    </button>
                </div>

                {MENUS
                    .into_iter()
                    .map(|val| {
                        let val_temp = val.clone();
                        view! {
                            <a
                                href=format!(
                                    "{}",
                                    val_temp.clone().1.to_string().replace(" ", "-").to_lowercase(),
                                )
                                class="btn btn-ghost btn-soft"
                            >
                                <div class="w-42 mx-auto flex items-center gap-2">
                                    <Icon
                                        icon=val.clone().0
                                        attr:class=move || {
                                            format!(
                                                "h-6 w-6 {}",
                                                if !menu_state.get().state { "mx-auto" } else { "" },
                                            )
                                        }
                                    />
                                    {move || {
                                        if menu_state.get().state {
                                            Either::Left(
                                                view! { <span>{val.clone().1.to_string()}</span> },
                                            )
                                        } else {
                                            Either::Right(())
                                        }
                                    }}
                                </div>
                            </a>
                        }
                            .into_any()
                    })
                    .collect_view()}
            </div>
        </aside>
    }
    .into_any()
}

#[derive(Debug, Clone)]
enum SidebarMenus {
    Dashboard,
    Packages,
    PromoPackages,
    Itinerary,
    Locations,
}

impl Display for SidebarMenus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SidebarMenus::Dashboard => write!(f, "Dashboard"),
            SidebarMenus::Packages => write!(f, "Packages"),
            SidebarMenus::PromoPackages => write!(f, "Promo Packages"),
            SidebarMenus::Itinerary => write!(f, "Itinerary"),
            SidebarMenus::Locations => write!(f, "Location"),
        }
    }
}

const MENUS: [(IconData, SidebarMenus); 5] = [
    (HOUSE, SidebarMenus::Dashboard),
    (PACKAGE, SidebarMenus::Packages),
    (TREASURE_CHEST, SidebarMenus::PromoPackages),
    (CLIPBOARD_TEXT, SidebarMenus::Itinerary),
    (MAP_PIN_AREA, SidebarMenus::Locations),
];
