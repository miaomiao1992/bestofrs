use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn TagTableSkeleton() -> Element {
    rsx! {
        div { class: "space-y-3",
            Skeleton { class: "skeleton h-[52px] w-full border border-primary-6" }
            Skeleton { class: "skeleton h-[1018px] w-full border border-primary-6" }
        }
    }
}
