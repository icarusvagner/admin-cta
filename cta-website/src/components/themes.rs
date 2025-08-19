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
			<div
				tabindex="0"
				class="overflow-y-auto right-0 top-px mt-2 w-52 border shadow-2xl z-1 dropdown-content bg-base-300 text-base-content rounded-box h-[30.5rem] max-h-[calc(100vh-8.6rem)] border-white/5 outline-1 outline-black/5"
			>
				<ul class="w-full menu menu-sm">
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
		</div>
	}.into_any()
}
