use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::{context_provider::{ConfigProvider, Tokens}, error::{Error, Result}, server::auth::api_login_req, types::request_types::{LoginPayload, LoginReturn}};

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
                leptos::logging::log!("Something went wrong: {}", ex.to_string());
                Err(Error::Network(ex.to_string()))
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

    let submit_form = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let navigate =navigate.clone();

        spawn_local(async move {
            match send_login_api(form_input.get().username, form_input.get().password).await {
                Ok(res) => {
                    let result = res.result;
                    if result.success {
                        let tokens = Tokens {
                            access_token: result.access_token,
                            refresh_token: result.refresh_token
                        };
                        config_context.set_tokens(tokens);
                        navigate("/", Default::default());
                    }
                },
                Err(ex) => {
                    leptos::logging::log!("Something went wrong: {}", ex.to_string());
                    result_err.set(ex.to_string());
                }
            }
        });
    };
    view! {
        <section class="flex min-h-screen flex-col items-center justify-center">
            <form
                on:submit=submit_form
                class="mx-auto w-xl rounded p-10 border border-base-100 bg-base-200 space-y-6"
            >
                <div class="flex items-center gap-5 justify-between">
                    <img src="public/emblem-logo.png" alt="CTA logo" class="h-24" />
                    <h1 class="text-2xl font-bold tracking-wide">
                        "Signin to your CTA admin account."
                    </h1>
                </div>
                <div class="flex flex-col gap-0.5">
                    <label class="floating-label">
                        <input
                            autocomplete="username"
                            prop:value=move || form_input.get().username
                            on:input=move |e| {
                                let mut current = form_input.get();
                                current.username = event_target_value(&e);
                                form_input.set(current);
                            }
                            type="text"
                            placeholder="Username *"
                            id="username"
                            class="validator input input-lg w-full"
                            required
                            pattern=r#"^(?=.{3,16}$)[a-zA-Z0-9_]+$"#
                            minlength="3"
                            maxlength="30"
                            title="Only letters, numbers or dash"
                        />
                        <span>"Username *"</span>

                    // {move || {
                    // if !form_input.get().username.is_empty() {
                    // Either::Left(
                    // view! {
                    // <p class="validator-hint">
                    // "Must be 3 to 30 characters" <br />
                    // "containing only letters, numbers or dash"
                    // </p>
                    // },
                    // )
                    // } else {
                    // Either::Right(())
                    // }
                    // }}
                    </label>
                </div>

                <div class="flex flex-col gap-0.5">
                    <label class="floating-label">
                        <input
                            autocomplete="new-password"
                            prop:value=move || form_input.get().password
                            on:input=move |e| {
                                let mut current = form_input.get();
                                current.password = event_target_value(&e);
                                form_input.set(current);
                            }
                            type="password"
                            class="validator input input-lg w-full"
                            required
                            placeholder="Password *"
                            minlength="8"
                            pattern=r#"(?=.*\d)(?=.*[a-z]).{8,}"#
                            title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
                        />
                        <span>"Password *"</span>

                    // {move || {
                    // if !form_input.get().password.is_empty() {
                    // Either::Left(
                    // view! {
                    // <p class="validator-hint">
                    // "Must be more than 8 characters, including" <br />
                    // "At least one number " <br />
                    // "At least one lowercase letter "<br />
                    // "At least one uppercase letter"
                    // </p>
                    // },
                    // )
                    // } else {
                    // Either::Right(())
                    // }
                    // }}
                    </label>
                </div>

                <button
                    type="submit"
                    class="btn w-full btn-primary text-xl font-semibold tracking-wide"
                >
                    "Submit"
                </button>
            </form>
        </section>
    }.into_any()
}
