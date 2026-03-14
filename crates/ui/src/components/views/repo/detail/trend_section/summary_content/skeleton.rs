use dioxus::prelude::*;

use crate::components::icons::RustGearIcon;
use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn TrendSummarySkeleton() -> Element {
    rsx! {
        div { class: "space-y-4",
            div { class: "space-y-2",
                Skeleton { class: "skeleton h-12 w-48 rounded-sm" }
                Skeleton { class: "skeleton h-8 w-40 rounded-sm" }
            }
            div { class: "flex flex-wrap justify-center gap-2 mb-8",
                for _ in 0..3 {
                    Skeleton { class: "skeleton h-8 w-20 rounded-sm" }
                }
            }
        }
        div { class: "grid grid-cols-1 gap-6 md:grid-cols-3 w-[64%] mx-auto",
            for _ in 0..3 {
                div { class: "relative flex min-h-[230px] items-center justify-center",
                    RustGearIcon {
                        width: 180.0,
                        class: "absolute text-primary-6",
                    }
                    div { class: "relative z-10 flex flex-col items-center gap-2 text-center",
                        Skeleton { class: "skeleton h-4 w-16 rounded-sm" }
                        Skeleton { class: "skeleton h-10 w-24 rounded-sm" }
                    }
                }
            }
        }
    }
}
