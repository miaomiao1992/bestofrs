use dioxus::prelude::*;

use crate::components::icons;
use crate::root::theme::toggle_theme;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut is_dark = use_signal(|| false);

    use_effect(move || {
        let mut is_dark = is_dark;
        spawn(async move {
            let value = crate::root::theme::is_dark_mode().await;
            is_dark.set(value);
        });
    });

    rsx! {
        button {
            class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 p-2 text-secondary-5 transition-colors hover:bg-primary-3 hover:text-secondary-4",
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
