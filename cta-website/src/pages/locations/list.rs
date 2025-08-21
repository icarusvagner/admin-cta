use leptos::{either::Either, prelude::*, task::spawn_local};

use crate::{error::{Error, Result}, server::location::api_get_locations, types::location::{ListLocationReturn, OneLocation}, utils::time::format_date_time};

async fn get_list_locations() -> Result<ListLocationReturn> {
	match api_get_locations().await {
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

	spawn_local(async move {
		let locs = get_list_locations().await;
		match locs {
			Ok(res) => {
				// leptos::logging::log!("{:#?}", res.result);
				list_loc.set(res.result.data);
			}
			Err(err) => {
				result_err.set(err.to_string());
			}
		}
	});

    view! {
		<div class="overflow-x-auto">
			{move || {
				if list_loc.get().len().lt(&1) {
					Either::Left(
						view! {
							<div class="flex flex-col justify-center items-center p-12 w-full">
								<h1 class="text-4xl font-bold tracking-wider">
									"No data are retrieved"
								</h1>
							</div>
						},
					)
				} else {
					Either::Right(
						view! {
							<table class="table">
								<thead>
									<tr>
										<th>"Loc #"</th>
										<th class="w-32">"Name"</th>
										<th class="w-32">"City"</th>
										<th class="w-32">"Province"</th>
										<th class="w-32">"Category"</th>
										<th>"Description"</th>
										<th class="w-32">"Created"</th>
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
