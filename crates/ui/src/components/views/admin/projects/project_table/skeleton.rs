use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn ProjectTableSkeleton() -> Element {
    rsx! {
        div { class: "space-y-3",
            Skeleton { class: "skeleton h-[1034px] w-full border border-primary-6" }
        }
    }
}
