use leptos::{either::{ EitherOf3}, prelude::*, task::spawn_local, html, ev, wasm_bindgen::JsCast, web_sys::*};
use leptos_use::use_event_listener;
use phosphor_leptos::{Icon, TRASH};

use crate::{components::loading::LoadingComponent, error::{Error, Result}, server::location::{api_get_locations, api_remove_location}, types::location::{ListLocationReturn, OneLocation}, utils::time::format_date_time};

async fn get_list_locations() -> Result<ListLocationReturn> {
	match api_get_locations().await {
		Ok(res) => Ok(res),
		Err(err) => {
            leptos::logging::log!("{}", err.to_string());
            Err(Error::Network(err.to_string()))
		}
	}
}

async fn remove_one_location(id: i64) -> Result<serde_json::Value> {
	match api_remove_location(id).await {
		Ok(res) => Ok(res),
		Err(err) => {
            leptos::logging::log!("{}", err.to_string());
            Err(Error::Network(err.to_string()))
		}
	}
}

#[component]
pub fn LocationList() -> AnyView {
	let list_loc = RwSignal::new(Vec::<OneLocation>::default());
	let result_err = RwSignal::new(String::new());
	let loading_state = RwSignal::new(false);
	let table_ref = NodeRef::<html::Table>::new();
	let id_remove = RwSignal::new(0);

	spawn_local(async move {
		loading_state.set(true);
		let locs = get_list_locations().await;

		match locs {
			Ok(res) => {
				// leptos::logging::log!("{:#?}", res.result);
				list_loc.set(res.result.data);
				loading_state.set(false);
			}
			Err(err) => {
				result_err.set(err.to_string());
				loading_state.set(false);
			}
		}
	});

	let _ = use_event_listener(table_ref, ev::click, move |evt: ev::MouseEvent| {

		if let Some(element) = evt.target().and_then(|t| t.dyn_into::<Element>().ok()) {
			let tag = element.tag_name().to_uppercase();
			let tag_id = element.id();
			if tag.eq(&"BUTTON") || tag.eq(&"SVG") {
				let id = tag_id.split('-').next_back().unwrap_or_default();
				id_remove.set(id.parse::<i64>().unwrap_or_default());
				let _ = element.set_attribute("disabled", "");

				spawn_local(async move {
					match remove_one_location(id_remove.get()).await {
						Ok(res) => {
							list_loc.update(|v| {
								v.retain(|s| s.id != id_remove.get());
							});
							leptos::logging::log!("{:?}", res);
						}
						Err(err) => {
							leptos::logging::error!("{:?}", err.to_string());
							result_err.set(err.to_string());
						}
					}
				});
			}
		}
	});

    view! {
		<div class="overflow-x-auto">
			{move || {
				if loading_state.get() {
					EitherOf3::A(view! { <LoadingComponent /> })
				} else if list_loc.get().len().lt(&1) {
					EitherOf3::B(
						view! {
							<div class="flex flex-col justify-center items-center p-12 w-full">
								<h1 class="text-4xl font-bold tracking-wider">
									"No data are retrieved"
								</h1>
							</div>
						},
					)
				} else {
					EitherOf3::C(
						view! {
							<table class="table" node_ref=table_ref>
								<thead>
									<tr>
										<th>"Loc #"</th>
										<th class="w-32">"Name"</th>
										<th class="w-32">"City"</th>
										<th class="w-32">"Province"</th>
										<th class="w-32">"Category"</th>
										<th>"Description"</th>
										<th class="w-32">"Created"</th>
										<th></th>
									</tr>
								</thead>
								<tbody>
									<For each=move || list_loc.get() key=|loc| loc.id let:child>
										<tr>
											<th>{child.id}</th>
											<td class="capitalize">{child.name}</td>
											<td class="capitalize">{child.city}</td>
											<td class="capitalize">{child.province}</td>
											<td class="capitalize">{child.category}</td>
											<td>{child.description}</td>
											<td>{format_date_time(child.ctime)}</td>
											<td>
												<div class="tooltip tooltip-left" data-tip="Remove Item">
													<button
														class="btn btn-sm btn-soft btn-error"
														id=format!("table-{}", child.id)
													>
														<Icon
															icon=TRASH
															attr:class="w-5 h-5"
															attr:id=format!("table-{}", child.id)
														/>
													</button>
												</div>
											</td>
										</tr>
									</For>
								</tbody>
							</table>
						},
					)
				}
			}}
		</div>
	}
    .into_any()
}
