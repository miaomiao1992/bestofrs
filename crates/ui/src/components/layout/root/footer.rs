use dioxus::prelude::*;

use super::gearmap::GearMap;
use crate::{
    components::icons::{FerrisIcon, RustGearIcon},
    root::Route,
};

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "relative mt-auto min-h-[420px]",
            div { class: "pointer-events-none absolute inset-x-0 bottom-0 h-[560px] overflow-hidden -z-0",
                GearMap {
                    count: 7,
                    class: "text-primary-6/70",
                    height: 560,
                }
            }

            footer {
                class: "relative z-10 min-h-[420px] border-t border-primary-6 bg-transparent overflow-hidden",
                div { class: "mx-auto flex w-full max-w-6xl flex-col gap-3 px-4 py-5 text-xs text-secondary-5 md:flex-row md:items-center md:justify-between",
                    div { class: "flex items-center gap-2",
                        span { class: "font-semibold tracking-tight text-secondary-4", "bestofrs" }
                        span { class: "text-secondary-6", "· Curated Rust ecosystem discovery" }
                        FerrisIcon { height: 40.0 }
                        RustGearIcon { width: 40.0 }
                    }
                    nav { class: "flex items-center gap-4",
                        Link { class: "hover:text-secondary-4 hover:underline", to: Route::HomeView {}, "Home" }
                        Link { class: "hover:text-secondary-4 hover:underline", to: Route::RepoListView {}, "Repo" }
                        Link { class: "hover:text-secondary-4 hover:underline", to: Route::TagListView {}, "Tag" }
                    }
                }
            }
        }
    }
}
