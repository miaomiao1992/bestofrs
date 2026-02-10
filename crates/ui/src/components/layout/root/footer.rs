use dioxus::prelude::*;

use crate::root::Route;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "mt-auto border-t border-primary-6 bg-primary-2/85 backdrop-blur",
            div { class: "mx-auto flex w-full max-w-6xl flex-col gap-3 px-4 py-5 text-xs text-secondary-5 md:flex-row md:items-center md:justify-between",
                div { class: "flex items-center gap-2",
                    span { class: "font-semibold tracking-tight text-secondary-4", "bestofrs" }
                    span { class: "text-secondary-6", "· Curated Rust ecosystem discovery" }
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
