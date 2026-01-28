use crate::{
    components::{icons, toast::ToastProvider},
    root::routes::Route,
    root::theme::{is_dark_mode, theme_seed, toggle_theme},
};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    let mut is_dark = use_signal(|| false);
    use_effect(move || {
        theme_seed();
        let mut is_dark = is_dark;
        spawn(async move {
            let value = is_dark_mode().await;
            is_dark.set(value);
        });
    });

    rsx! {
        ToastProvider {
            header { class: "border-b border-primary-6 bg-primary-2",
                div { class: "mx-auto max-w-6xl px-4 py-3 flex items-center justify-between",
                    div { class: "flex items-center gap-4",
                        Link { class: "font-semibold", to: Route::Home {}, "bestofrs" }
                        nav { class: "flex items-center gap-3 text-sm",
                            Link { class: "text-secondary-5 hover:underline", to: Route::Home {}, "Home" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::RepoList {}, "Repo" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::TagList {}, "Tag" }
                            Link { class: "text-secondary-5 hover:underline", to: Route::Admin {}, "Admin" }
                        }
                    }
                    div { class: "flex items-center gap-3",
                        button {
                            class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 p-2 text-secondary-5 hover:bg-primary-4",
                            onclick: move |_| {
                                toggle_theme();
                                is_dark.set(!is_dark());
                            },
                            aria_label: "Toggle theme",
                            if is_dark() {
                                icons::MoonIcon { size: 18 }
                            } else {
                                icons::SunIcon { size: 18 }
                            }
                        }
                    }
                }
            }

            main { class: "min-h-screen",
                Outlet::<Route> {}
            }
        }
    }
}
