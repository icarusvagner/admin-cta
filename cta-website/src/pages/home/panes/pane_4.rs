use leptos::{prelude::*, reactive::spawn_local};
use crate::{error::{Error, Result}, server::location::api_count_locations, types::location::CountLocationReturn};

async fn get_count_locs() -> Result<CountLocationReturn> {
	match api_count_locations().await {
		Ok(res) => Ok(res),
		Err(err) => {
			leptos::logging::error!("{}", err.to_string());
			Err(Error::Network(err.to_string()))
		}
	}
}

#[component]
pub fn Pane4() -> AnyView {
	let count_loc = RwSignal::new(0);

	spawn_local(async move {
		match get_count_locs().await {
			Ok(res) => {
				count_loc.set(res.result.data);
			},
			Err(err) => {
				leptos::logging::error!("{}", err.to_string());
			}
		}
	});

    view! {
		<div class="flex flex-col gap-5 justify-center items-center p-12 rounded border-accent bg-base-300">
			<div class="mx-auto w-full mm:w-sm">
				<h1 class="text-4xl font-black tracking-wide">"Locations"</h1>
				<div class="text-8xl tabular-nums">
					{move || {
						if format!("{}", count_loc.get()).chars().count().gt(&2) {
							format!("{}", count_loc.get())
						} else {
							format!("0{}", count_loc.get())
						}
					}}
				</div>
			</div>
		</div>
	}.into_any()
}
