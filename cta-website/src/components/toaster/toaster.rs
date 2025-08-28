use leptos::prelude::*;

use crate::components::toaster::{toast::data::ToastPosition, ToasterContext};

const CONTAINER_POSITION: &[ToastPosition] = &[
    ToastPosition::Top,
    ToastPosition::TopLeft,
    ToastPosition::TopRight,
    ToastPosition::Bottom,
    ToastPosition::BottomLeft,
    ToastPosition::BottomRight,
];

#[component]
pub fn Toaster(#[prop(into, optional)] stacked: Signal<bool>) -> AnyView {
    view! {}.into_any()
}

pub fn provide_toaster() {
    if use_context::<ToasterContext>().is_none() {
        provide_context(ToasterContext::default());
    }
}

#[must_use]
pub fn expect_toaster() -> ToasterContext {
    expect_context::<ToasterContext>()
}

fn is_container_empty(position: &ToastPosition) -> bool {
    !expect_toaster()
        .queue
        .get()
        .iter()
        .any(|toast| toast.position.eq(position))
}

fn get_container_id(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft => "absolute inset-2",
        ToastPosition::Top => "absolute top-2 left-1/2 -translate-x-1/2",
        ToastPosition::TopRight => "absolute top-2 right-2",
        ToastPosition::BottomLeft => "absolute bottom-2 left-2",
        ToastPosition::Bottom => "absolute left-1/2 bottom-2 -translate-x-1/2",
        ToastPosition::BottomRight => "absolute bottom-2 right-2",
    }
}
