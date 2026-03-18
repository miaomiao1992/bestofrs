use dioxus::prelude::*;

#[component]
pub(super) fn Tip() -> Element {
    rsx! {
        p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
            "用于本地开发：手动触发一次 ingest（production 环境会返回 403）。"
        }
    }
}
