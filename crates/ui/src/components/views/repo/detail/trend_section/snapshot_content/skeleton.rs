use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn SnapshotContentSkeleton() -> Element {
    rsx! {
        div { class: "flex h-full flex-col gap-2",
            Skeleton { class: "skeleton h-4 w-28 rounded-sm" }
            div { class: "min-h-0 flex-1 border border-primary-6 bg-primary p-3",
                Skeleton { class: "skeleton h-full w-full rounded-sm" }
            }
        }
    }
}
