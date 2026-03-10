use dioxus::prelude::*;

use super::{rank_desc, rank_title, RankType};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeRankTabItemProps {
    idx: usize,
    tab: RankType,
    active_tab: RankType,
    on_select: EventHandler<MouseEvent>,
}

#[component]
pub(super) fn HomeRankTabItem(props: HomeRankTabItemProps) -> Element {
    let is_active = props.active_tab == props.tab;
    rsx! {
        div {
            class: if is_active {
                "rounded-2xl transition-all duration-500 relative flex flex-col bg-primary-1 shadow-sm flex-grow"
            } else {
                "rounded-2xl transition-all duration-500 relative flex flex-col bg-transparent hover:bg-primary-1/40"
            },
            if is_active {
                div { class: "absolute left-0 top-0 bottom-0 w-1 bg-secondary-6" }
            }
            button {
                onclick: move |e| props.on_select.call(e),
                class: "w-full px-6 py-6 flex items-center justify-between group text-left relative z-10",
                div { class: "flex flex-col items-start",
                    span { class: "text-[10px] font-mono uppercase tracking-[0.2em] text-secondary-5 mb-1",
                        "M_0{props.idx + 1}"
                    }
                    span {
                        class: if is_active {
                            "text-lg font-black font-sans uppercase tracking-widest transition-colors text-secondary-2"
                        } else {
                            "text-lg font-black font-sans uppercase tracking-widest transition-colors text-secondary-5 group-hover:text-secondary-2"
                        },
                        "{rank_title(props.tab)}"
                    }
                }
                span {
                    class: if is_active {
                        "transition-all duration-300 rotate-90 text-secondary-6"
                    } else {
                        "transition-all duration-300 text-primary-6"
                    },
                    "›"
                }
            }
            div {
                class: if is_active {
                    "px-6 flex-grow flex items-start overflow-hidden transition-all duration-700 ease-in-out opacity-100 pb-12"
                } else {
                    "px-6 flex-grow flex items-start overflow-hidden transition-all duration-700 ease-in-out max-h-0 opacity-0"
                },
                div { class: "relative pt-2",
                    div { class: "absolute -left-3 top-0 bottom-0 w-[2px]", style: "background: color-mix(in oklab, var(--grid-accent) 30%, transparent);" }
                    p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed pl-4",
                        "{rank_desc(props.tab)}"
                    }
                }
            }
        }
    }
}
