use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_navigate;
use phosphor_leptos::{Icon, GEAR, MAGNIFYING_GLASS, SIGN_OUT, USER};

use crate::{components::themes::ThemeSwitcher, context_provider::ConfigProvider, error::{Error, Result}, server::auth::api_logout_req, types::request_types::{LogoffPayload, LogoffReturn}};

async fn send_logoff_api(logoff: bool) -> Result<LogoffReturn> {
	let data = LogoffPayload { logoff };

	match api_logout_req(data).await {
	    Ok(res) => Ok(res),
	    Err(ex) => {
	                leptos::logging::log!("{}", ex.to_string());
	                Err(Error::Network(ex.to_string()))
	    }
	} 
}

#[component]
pub fn NavbarMenu() -> AnyView {
	let result_err = RwSignal::new(String::new());
	let navigate = use_navigate();
	let mut context_config = ConfigProvider::expect_context();

	let log_out = move |_| {
		let nav = navigate.clone();
		
		spawn_local(async move {
			match send_logoff_api(true).await {
				Ok(res) => {
					if res.result.logged_off {
						context_config.logout();
						nav("/login", Default::default());
					}
				},
				Err(ex) => {
					result_err.set(ex.to_string());
				}
			}
		});
	};

    view! {
		<div class="shadow-sm navbar bg-base-300">
			<div class="ml-12 navbar-start">
				<a class="text-xl font-black tracking-wider btn btn-ghost font-quicksand">
					"CTA Admin"
				</a>
			</div>

			<div class="navbar-center">
				<label class="input w-md">
					<input type="text" placeholder="Search..." />
					<span class="label">
						<Icon icon=MAGNIFYING_GLASS attr:class="h-5 w-5" />
					</span>
				</label>
			</div>

			<div class="navbar-end">
				<ThemeSwitcher />
				<div class="dropdown">

					<div
						tabindex="0"
						role="button"
						class="btn btn-ghost btn-circle"
					>
						<Icon icon=GEAR attr:class="h-5 w-5" />
					</div>
					<ul
						tabindex="0"
						class="right-0 p-2 mt-2 w-52 shadow menu menu-sm dropdown-content bg-base-300 rounded-box z-1"
					>
						<li>
							<a href="/profile" class="btn btn-ghost">
								<Icon icon=USER attr:class="h-5 w-5" />
								<span>"Profile"</span>
							</a>
						</li>
						<li>
							<button class="btn btn-ghost" on:click=log_out>
								<Icon icon=SIGN_OUT attr:class="h-5 w-5" />
								<span>"Logout"</span>
							</button>
						</li>
					</ul>
				</div>
			</div>
		</div>
	}
    .into_any()
}
