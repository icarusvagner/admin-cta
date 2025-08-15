use leptos::{either::Either, prelude::*};

#[derive(Default, Clone, Debug)]
struct LoginField {
    username: String,
    password: String,
}

#[component]
pub fn LoginPage() -> AnyView {
    let form_input = RwSignal::new(LoginField::default());

    view! {
        <section class="flex min-h-screen flex-col items-center justify-center">
            <form action="#" class="mx-auto w-xl rounded p-5 space-y-6">
                <h1 class="mb-8 text-2xl font-bold tracking-wide">
                    "Signin to your CTA admin account."
                </h1>
                <div class="flex flex-col gap-0.5">
                    <label class="floating-label">
                        <input
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
                            pattern=r#"[A-Za-z][A-Za-z0-9\-]*"#
                            minlength="3"
                            maxlength="30"
                            title="Only letters, numbers or dash"
                        />
                        <span>"Username *"</span>

                        {move || {
                            if !form_input.get().username.is_empty() {
                                Either::Left(
                                    view! {
                                        <p class="validator-hint">
                                            "Must be 3 to 30 characters" <br />
                                            "containing only letters, numbers or dash"
                                        </p>
                                    },
                                )
                            } else {
                                Either::Right(())
                            }
                        }}
                    </label>
                </div>

                <div class="flex flex-col gap-0.5">
                    <label class="floating-label">
                        <input
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
                            pattern=r#"(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"#
                            title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
                        />
                        <span>"Password *"</span>

                        {move || {
                            if !form_input.get().password.is_empty() {
                                Either::Left(
                                    view! {
                                        <p class="validator-hint">
                                            "Must be more than 8 characters, including" <br />
                                            "At least one number " <br />
                                            "At least one lowercase letter "<br />
                                            "At least one uppercase letter"
                                        </p>
                                    },
                                )
                            } else {
                                Either::Right(())
                            }
                        }}
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
