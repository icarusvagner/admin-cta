use std::fmt::Display;

use leptos::{either::Either, prelude::*, reactive::spawn_local, html, ev, web_sys::*, wasm_bindgen::*};
use leptos_router::hooks::{use_location, use_navigate};
use leptos_use::use_event_listener;
use phosphor_leptos::{
    Icon, IconData, CARET_DOUBLE_LEFT, CARET_DOUBLE_RIGHT, CLIPBOARD_TEXT, HOUSE, MAP_PIN_AREA,
    PACKAGE, SIGN_OUT, TREASURE_CHEST,
};

use crate::{components::send_logoff_api, context_provider::ConfigProvider};

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
	let result_err = RwSignal::new(String::new());
	let navigate = use_navigate();
	let mut context_config = ConfigProvider::expect_context();
	let btn_state = RwSignal::new(false);
	let path = use_location().pathname;

	let ext_class = RwSignal::new(String::new());

	let log_out = move |_| {
		let nav = navigate.clone();
		btn_state.set(true);
		
		spawn_local(async move {
			match send_logoff_api(true).await {
				Ok(res) => {
					if res.result.logged_off {
						btn_state.set(false);
						context_config.logout();
						nav("/login", Default::default());
					}
				},
				Err(ex) => {
					result_err.set(ex.to_string());
					btn_state.set(false);
				}
			}
		});
	};


    view! {
		<aside class=move || {
			format!(
				"fixed top-0 left-0 bg-base-300 duration-300 min-h-screen transition-all ease-in-out flex flex-col {}",
				menu_state.get().class,
			)
		}>
			<div class="flex relative flex-col gap-4 w-full h-full">
				<a href="/" class="flex gap-2 items-center py-2.5 px-3 w-full">
					<img
						src="/public/emblem-logo.png"
						class="h-16"
						alt="horizontal cta logo"
					/>
					{move || {
						if menu_state.get().state {
							Either::Left(
								view! {
									<span class="text-xl font-bold tracking-tight transition-all duration-300 ease-in-out delay-300 font-quicksand">
										"Cebu Tours & Adventures"
									</span>
								},
							)
						} else {
							Either::Right(())
						}
					}}
				</a>

				<div class="absolute top-3 -right-7">
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
									view! {
										<Icon icon=CARET_DOUBLE_RIGHT attr:class="h-6 w-6" />
									},
								)
							} else {
								Either::Right(
									view! {
										<Icon icon=CARET_DOUBLE_LEFT attr:class="h-6 w-6" />
									},
								)
							}
						}}
					</button>
				</div>
			</div>

			<div
				class="flex flex-col w-full h-full"
				id="menu-links"
			>
				{MENUS
					.into_iter()
					.map(|(icon, link)| {
						let link = RwSignal::new(link);
						ext_class
							.update(|val| {
								if path.get().eq(&link.get().to_link()) {
									*val = "menu-active".to_string()
								} else {
									*val = "menu-active-hovered".to_string()
								}
							});

						view! {
							<a
								href=format!("{}", link.get().to_link())
								class=format!(
									"flex gap-2 items-center py-2.5 px-3 {}",
									ext_class.get(),
								)
								id=link.get().to_string().to_lowercase()
							>
								<div class="flex gap-2 items-center mx-auto w-42">
									<Icon
										icon=icon
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
												view! { <span>{link.get().to_string()}</span> },
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

			<hr class="mt-4 h-0.5 text-base-100" />
			<div class="my-auto"></div>

			<div class="p-3 w-full">
				<button
					on:click=log_out
					class="flex gap-2 items-center w-full btn btn-primary"

					disabled=move || btn_state.get()
				>
					{move || {
						if !btn_state.get() {
							Either::Left(
								view! {
									<Icon icon=SIGN_OUT attr:class="h-5 w-5" />
									{move || {
										if menu_state.get().state {
											Either::Left(
												view! {
													<span class="transition-all duration-300 ease-in delay-400">
														"Log Out"
													</span>
												},
											)
										} else {
											Either::Right(())
										}
									}}
								},
							)
						} else {
							Either::Right(
								view! {
									<span class="loading loading-spinner"></span>
									<span class=format!(
										"{}",
										if !menu_state.get().state { "hidden" } else { "" },
									)>"Loading"</span>
								},
							)
						}
					}}
				</button>
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

impl SidebarMenus {
    fn to_link(&self) -> String {
        match self {
            SidebarMenus::Dashboard => "/".into(),
            SidebarMenus::Packages => "/packages".into(),
            SidebarMenus::PromoPackages => "/promo-packages".into(),
            SidebarMenus::Itinerary => "/itinerary".into(),
            SidebarMenus::Locations => "/location".into(),
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
