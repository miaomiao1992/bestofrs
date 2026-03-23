use dioxus::prelude::*;

use super::{RepoListContext, RepoListHeroType};

#[component]
pub(super) fn RepoMeta() -> Element {
    let ctx = use_context::<RepoListContext>();
    let hero_type = if (ctx.active_tags)().is_empty() {
        RepoListHeroType::AllProjects
    } else {
        RepoListHeroType::SearchResult
    };

    rsx! {
        div { class: "max-w-3xl px-1 md:px-0",
            h1 { class: "mb-1.5 flex flex-wrap items-center gap-1.5 text-xl font-black font-sans uppercase tracking-tight text-secondary-2 md:mb-2 md:gap-2 md:text-3xl",
                if hero_type == RepoListHeroType::SearchResult {
                    "Search Result"
                } else {
                    "All Project"
                }
            }
            h2 { class: "font-mono text-xs italic leading-relaxed text-secondary-3 md:text-base",
                "A curated collection of the Rust ecosystem. Monitor things growth, track update velocity, and discover the core building blocks of Rust."
            }
        }
    }
}
