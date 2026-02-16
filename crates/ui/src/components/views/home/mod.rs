use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "relative min-h-[70vh] overflow-hidden rounded-xl border border-primary-6 bg-primary-1",
            div { class: "relative z-10 py-8 space-y-6",
                div { class: "rounded-xl border border-primary-6 bg-primary-2/90 px-6 py-6 space-y-2 mx-6 mt-6",
                    h1 { class: "text-2xl font-semibold tracking-tight text-secondary-4", "Home" }
                    p { class: "text-sm text-secondary-5", "Repo list" }
                }
            }
        }
    }
}
