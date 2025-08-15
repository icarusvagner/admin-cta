use cta_website::App;
use leptos::prelude::*;

fn main() {
    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(
            tracing_subscriber_wasm::MakeConsoleWriter::default()
                .map_info_level_to(tracing::Level::DEBUG),
        )
        .init();

    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
