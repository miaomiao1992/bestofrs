use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridLineType, GridPadding, GridPattern, GridType,
    GridWrapper,
};
use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn MetaSectionSkeleton() -> Element {
    rsx! {
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex flex-col gap-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12",
                    GridWrapper {
                        class: "lg:col-span-9".to_string(),
                        padding: GridPadding::Sm,
                        is_dot_on: false,
                        div { class: "flex min-w-0 flex-col gap-8",
                            div { class: "flex flex-col items-start gap-8 md:flex-row",
                                div { class: "relative h-32 w-32 shrink-0 md:h-40 md:w-40",
                                    div { class: "absolute left-2 top-2 h-full w-full border-2 border-primary-6 bg-screentone" }
                                    Skeleton { class: "skeleton relative z-10 h-32 w-32 border-4 border-primary-6 md:h-40 md:w-40" }
                                }
                                div { class: "flex min-w-0 flex-1 flex-col gap-4",
                                    Skeleton { class: "skeleton h-16 w-56 border border-primary-6 md:h-20 md:w-72" }
                                    Skeleton { class: "skeleton h-4 w-48 border border-primary-6" }
                                    Skeleton { class: "skeleton h-6 w-full max-w-3xl border border-primary-6" }
                                }
                            }

                            Skeleton { class: "skeleton h-3 w-56 border border-primary-6" }

                            GridWrapper {
                                grid_type: GridType::Inner,
                                line_type: GridLineType::Dashed,
                                is_dot_on: false,
                                padding: GridPadding::None,
                                background: GridBackground {
                                    pattern: GridPattern::Slash,
                                    gradient: GradientDirection::None,
                                },
                                div { class: "grid grid-cols-4 gap-2 px-2 py-2",
                                    Skeleton { class: "skeleton h-14 border border-primary-6" }
                                    Skeleton { class: "skeleton h-14 border border-primary-6" }
                                    Skeleton { class: "skeleton h-14 border border-primary-6" }
                                    Skeleton { class: "skeleton h-14 border border-primary-6" }
                                }
                            }
                        }
                    }

                    GridWrapper {
                        class: "lg:col-span-3".to_string(),
                        padding: GridPadding::Sm,
                        line_type: GridLineType::None,
                        div { class: "flex flex-col gap-6",
                            Skeleton { class: "skeleton h-4 w-28 border border-primary-6" }

                            div { class: "flex flex-col gap-5",
                                Skeleton { class: "skeleton h-[58px] w-[calc(100%-10px)] rounded-full border-4 border-primary-6" }
                                Skeleton { class: "skeleton h-[58px] w-[calc(100%-10px)] rounded-full border-4 border-primary-6" }
                            }

                            div { class: "flex flex-wrap gap-2 pt-1",
                                Skeleton { class: "skeleton h-7 w-20 border border-primary-6" }
                                Skeleton { class: "skeleton h-7 w-20 border border-primary-6" }
                                Skeleton { class: "skeleton h-7 w-24 border border-primary-6" }
                                Skeleton { class: "skeleton h-7 w-16 border border-primary-6" }
                            }
                        }
                    }
                }
            }
        }
    }
}
