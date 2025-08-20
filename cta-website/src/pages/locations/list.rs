use leptos::{prelude::*, task::spawn_local};

use crate::{error::{Error, Result}, server::location::api_get_location_by_id, types::request_types::LocationReturn};

async fn get_one_location(id: i64) -> Result<LocationReturn> {
    match api_get_location_by_id(id).await {
        Ok(res) => Ok(res),
        Err(err) => {
            leptos::logging::log!("{}", err.to_string());
            Err(Error::Network(err.to_string()))
        }
    }
}

#[component]
pub fn LocationList() -> AnyView {
	spawn_local(async move {
		let loc = get_one_location(1).await;
		leptos::logging::log!("{:?}", loc);
	});

    view! {
		<div class="overflow-x-auto">
			<table class="table">
				// <!-- head -->
				<thead>
					<tr>
						<th></th>
						<th>Name</th>
						<th>Job</th>
						<th>Favorite Color</th>
					</tr>
				</thead>
				<tbody>
					// <!-- row 1 -->
					<tr>
						<th>1</th>
						<td>Cy Ganderton</td>
						<td>Quality Control Specialist</td>
						<td>Blue</td>
					</tr>
					// <!-- row 2 -->
					<tr class="hover:bg-base-300">
						<th>2</th>
						<td>Hart Hagerty</td>
						<td>Desktop Support Technician</td>
						<td>Purple</td>
					</tr>
					// <!-- row 3 -->
					<tr>
						<th>3</th>
						<td>Brice Swyre</td>
						<td>Tax Accountant</td>
						<td>Red</td>
					</tr>
				</tbody>
			</table>
		</div>
	}
    .into_any()
}
