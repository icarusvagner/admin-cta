use leptos::prelude::*;
use phosphor_leptos::{Icon, DIAMONDS_FOUR};

use crate::{context_provider::ConfigProvider, types::theme::THEMES};

#[component]
pub fn ThemeSwitcher() -> AnyView {
    let config_context = ConfigProvider::expect_context();

    view! {
        <div class="dropdown">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                <Icon icon=DIAMONDS_FOUR attr:class="h-5 w-5" />
            </div>
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content bg-base-300 rounded-box z-1 mt-2 w-52 p-2 shadow right-0"
            >
                {THEMES
                    .into_iter()
                    .map(|v| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| {
                                        let val = v.clone();
                                        config_context.update_theme(val);
                                    }
                                    type="button"
                                    class="btn btn-ghost"
                                >
                                    {v.clone().to_title()}
                                </button>
                            </li>
                        }
                            .into_any()
                    })
                    .collect_view()}
            </ul>
        </div>
    }.into_any()
}
