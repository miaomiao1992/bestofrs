use dioxus::prelude::*;

#[component]
pub fn AdminStateHint(message: String) -> Element {
    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 text-sm text-secondary-5",
            "{message}"
        }
    }
}
