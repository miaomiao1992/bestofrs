use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn ResultSkeleton() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 gap-2",
            Skeleton { class: "skeleton h-5 w-full border border-primary-6" }
            Skeleton { class: "skeleton h-5 w-full border border-primary-6" }
            Skeleton { class: "skeleton h-5 w-full border border-primary-6" }
        }
    }
}
