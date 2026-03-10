use dioxus::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub(crate) fn RankPanelListSkeleton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5",
            for _ in 0..7 {
                div { class: "rank-card bg-primary border-l-4 border-y border-r border-primary-6 rounded-2xl p-3 h-[86px]",
                    div { class: "flex h-full items-center",
                        div { class: "w-10 shrink-0",
                            Skeleton { class: "skeleton h-5 w-6 rounded-sm" }
                        }
                        div { class: "mr-6 shrink-0",
                            Skeleton { class: "skeleton h-9 w-9 rounded-full" }
                        }
                        div { class: "flex-1 min-w-0 mr-6 space-y-2",
                            Skeleton { class: "skeleton h-3 w-2/5 rounded-sm" }
                            Skeleton { class: "skeleton h-3 w-3/5 rounded-sm" }
                        }
                        div { class: "shrink-0",
                            Skeleton { class: "skeleton h-4 w-14 rounded-sm" }
                        }
                    }
                }
            }
        }
    }
}
