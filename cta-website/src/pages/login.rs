use leptos::{either::Either, ev, prelude::*, task::spawn_local};
use leptos_router::{hooks::use_navigate, location::State};

use crate::{context_provider::ConfigProvider, error::{Error, Result}, server::auth::api_login_req, types::request_types::{LoginPayload, LoginReturn}};

#[derive(Default, Clone, Debug)]
struct LoginField {
    username: String,
    password: String,
}

async fn send_login_api(username: String, pwd: String) -> Result<LoginReturn> {
    if !username.is_empty() && !pwd.is_empty() {
        let data = LoginPayload { username, pwd };

        match api_login_req(data).await {
            Ok(res) => Ok(res),
            Err(ex) => {
                Err(Error::Forbidden(ex.to_string()))
            }
        }
    } else {
        Err(Error::EmptyInputs)?
    }
}

#[component]
pub fn LoginPage() -> AnyView {
    let form_input = RwSignal::new(LoginField::default());
    let result_err = RwSignal::new(String::new());
    let mut config_context = ConfigProvider::expect_context();
    let navigate = use_navigate();
    let btn_state = RwSignal::new(false);

    let submit_form = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let form_data = move || form_input.get();
        let nav = navigate.clone();
        btn_state.set(true);

        if !form_data().username.is_empty() && !form_data().password.is_empty() {
	        spawn_local(async move {
	            match send_login_api(form_data().username, form_data().password).await {
	                Ok(res) => {
	                    if res.result.success {
				            btn_state.set(false);
	                    	config_context.is_logged_in();
	                    	nav("/", leptos_router::NavigateOptions { resolve: true, replace: true, scroll: true, state: State::default() });
	                    } else {
	                        result_err.set("Invalid username or password".to_string());
				            btn_state.set(false);
	                    }
	                },
	                Err(ex) => {
	                    leptos::logging::log!("Something went wrong: {}", ex.to_string());
	                    result_err.set(ex.to_string());
			            btn_state.set(false);
	                }
	            }
	        });
        } else {
            result_err.set("Invalid inputs".to_string());
            btn_state.set(false);
        }
    };

    view! {
		<section class="flex justify-center items-center mx-auto min-h-screen w-5xl">
			<div class="grid grid-cols-2 w-full rounded-2xl bg-base-200">
				<div>
					<img
						src="/public/images/login-office-JBFguH2f.jpeg"
						alt="login office"
						class="w-full"
					/>
				</div>
				<div class="flex flex-col justify-center p-12 space-y-5">
					<h1 class="text-2xl font-medium tracking-wider">
						"Admin Login"
					</h1>
					<form
						class="space-y-8"
						on:submit=submit_form
						id="submit_form"
						name="submit_form"
					>
						<label for="username" class="flex flex-col gap-1">
							<span>"Username"</span>

							<input
								autocomplete="username"
								prop:value=move || form_input.get().username
								on:input=move |e| {
									let mut current = form_input.get();
									current.username = event_target_value(&e);
									form_input.set(current);
								}
								type="text"
								placeholder="admin123456"
								id="username"
								name="username"
								class="py-2 px-3 w-full rounded border outline-none validator border-base-300"
								required
								pattern=r#"^(?=.{3,16}$)[a-zA-Z0-9_]+$"#
								minlength="3"
								maxlength="30"
								title="Only letters, numbers or dash"
							/>
						</label>

						<label for="password" class="flex flex-col gap-1">
							<span>"Password"</span>

							<input
								autocomplete="new-password"
								prop:value=move || form_input.get().password
								on:input=move |e| {
									let mut current = form_input.get();
									current.password = event_target_value(&e);
									form_input.set(current);
								}
								type="password"
								id="password"
								name="password"
								class="py-2 px-3 w-full rounded border outline-none validator border-base-300"
								required
								placeholder="********"
								minlength="8"
								pattern=r#"(?=.*\d)(?=.*[a-z]).{8,}"#
								title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
							/>
						</label>

						<button
							type="submit"
							class="w-full text-xl font-semibold tracking-wide btn btn-primary"
							disabled=move || btn_state.get()
						>
							{move || {
								if !btn_state.get() {
									Either::Left(view! { <span>"Submit"</span> })
								} else {
									Either::Right(
										view! {
											<span class="loading loading-spinner"></span>
											"Loading"
										},
									)
								}
							}}
						</button>
					</form>
				</div>
			</div>
		</section>
	}.into_any()
}
