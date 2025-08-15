use leptos::prelude::*;
use phosphor_leptos::{Icon, GEAR, MAGNIFYING_GLASS, SIGN_OUT, USER};

use crate::components::themes::ThemeSwitcher;

#[component]
pub fn NavbarMenu() -> AnyView {
    view! {
        <div class="navbar bg-base-300 shadow-sm">
            <div class="ml-12 navbar-start">
                <a class="btn btn-ghost text-xl font-quicksand font-black tracking-wider">
                    "CTA Admin"
                </a>
            </div>

            <div class="navbar-center">
                <label class="input w-md">
                    <input type="text" placeholder="Search..." />
                    <span class="label">
                        <Icon icon=MAGNIFYING_GLASS attr:class="h-5 w-5" />
                    </span>
                </label>
            </div>

            <div class="navbar-end">
                <ThemeSwitcher />
                <div class="dropdown">

                    <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                        <Icon icon=GEAR attr:class="h-5 w-5" />
                    </div>
                    <ul
                        tabindex="0"
                        class="menu menu-sm dropdown-content bg-base-300 rounded-box z-1 mt-2 w-52 p-2 shadow right-0"
                    >
                        <li>
                            <a href="/profile" class="btn btn-ghost">
                                <Icon icon=USER attr:class="h-5 w-5" />
                                <span>"Profile"</span>
                            </a>
                        </li>
                        <li>
                            <button class="btn btn-ghost">
                                <Icon icon=SIGN_OUT attr:class="h-5 w-5" />
                                <span>"Logout"</span>
                            </button>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
    .into_any()
}
