use leptos::{prelude::*, task::spawn_local};

use crate::{error::{Error, Result}, server::location::api_create_location, types::request_types::{CreateLocationPayload, WithIdReturn}};

async fn send_create_location(data: CreateLocationField) -> Result<WithIdReturn> {
    let CreateLocationField { name, city, province, category, description } = data;
    let loc_c = CreateLocationPayload { name: name.to_string(), city: city.to_string(), province: province.to_string(), category: category.to_string(), description: description.to_string()};

    match api_create_location(loc_c).await {
        Ok(res) => Ok(res),
        Err(err) => {
        	leptos::logging::log!("{}", err.to_string());
            Err(Error::FailedToCreate(err.to_string()))
        }
    }
}

#[component]
pub fn LocationCreate() -> AnyView {
	let location_form = RwSignal::new(CreateLocationField::default());
	let result_err =RwSignal::new(String::new());

	let submit_create = move |_| {
		let form_data = move || location_form.get();

		if !form_data().name.is_empty() && !form_data().city.is_empty() && !form_data().province.is_empty() && !form_data().category.is_empty() && !form_data().description.is_empty() {
		spawn_local(async move {
			match send_create_location(form_data()).await {
				Ok(res) => {
					leptos::logging::log!("Created {}", res.result.id);
				}
				Err(err) => {
					result_err.set(err.to_string());
				}
				
			}
		});
		}
	};

    view! {
		<div class="grid grid-cols-2 gap-10 p-10">
			<label class="floating-label">
				<input
					prop:value=move || location_form.get().name
					on:input=move |e| {
						let mut curr = location_form.get();
						curr.name = event_target_value(&e);
						location_form.set(curr);
					}
					type="text"
					placeholder="Name *"
					id="loc_name"
					name="loc_name"
					class="w-full input input-lg"
				/>
				<span>"Name *"</span>
			</label>
			<label class="floating-label">
				<input
					prop:value=move || location_form.get().city
					on:input=move |e| {
						let mut curr = location_form.get();
						curr.city = event_target_value(&e);
						location_form.set(curr);
					}
					type="text"
					placeholder="City *"
					id="loc_city"
					name="loc_city"
					class="w-full input input-lg"
				/>
				<span>"City *"</span>
			</label>
			<label class="floating-label">
				<input
					prop:value=move || location_form.get().province
					on:input=move |e| {
						let mut curr = location_form.get();
						curr.province = event_target_value(&e);
						location_form.set(curr);
					}
					type="text"
					placeholder="Province *"
					id="loc_province"
					name="loc_province"
					class="w-full input input-lg"
				/>
				<span>"Province *"</span>
			</label>
			<label class="floating-label">
				<input
					prop:value=move || location_form.get().category
					on:input=move |e| {
						let mut curr = location_form.get();
						curr.category = event_target_value(&e);
						location_form.set(curr);
					}
					type="text"
					placeholder="Category * (Landmark, Church, Beach, Adventure, etc)"
					id="loc_cat"
					name="loc_cat"
					class="w-full input input-lg"
				/>
				<span>"Category *"</span>
			</label>
			<label class="col-span-2 floating-label">
				<textarea
					prop:value=move || location_form.get().description
					on:input=move |e| {
						let mut curr = location_form.get();
						curr.description = event_target_value(&e);
						location_form.set(curr);
					}
					placeholder="Description"
					id="loc_desc"
					name="loc_desc"
					class="w-full textarea textarea-lg"
				></textarea>
				<span>"Description"</span>
			</label>

			<button
				type="button"
				class="w-full text-lg btn btn-primary btn-soft"
				on:click=submit_create
			>
				"Create Location"
			</button>
		</div>
	}
    .into_any()
}

#[derive(Debug, Clone, Default)]
struct CreateLocationField {
    name: String,
    city: String,
    province: String,
    category: String,
    description: String,
}
