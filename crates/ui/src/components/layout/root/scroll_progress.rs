use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/scroll_progress.js"::mount_scroll_progress);

#[component]
pub fn ScrollProgress() -> Element {
    use_effect(move || {
        spawn(async move {
            let _ = mount_scroll_progress::<()>().await;
        });
    });

    rsx! {
        div { class: "absolute inset-x-0 top-0 h-0.5 bg-primary-6/40",
            div {
                id: "root-scroll-progress",
                class: "h-full w-0 bg-secondary-4 transition-[width] duration-75",
                role: "progressbar",
                aria_label: "Page scroll progress",
                aria_valuemin: "0",
                aria_valuemax: "100",
                aria_valuenow: "0",
            }
        }
    }
}
