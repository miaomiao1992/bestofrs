use dioxus::prelude::*;

use crate::components::common::{
    CommonMarkdown, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridWrapper, SEOHead, SEOProp,
};

const ABOUT_MD: &str = include_str!("./about.md");

#[component]
pub fn About() -> Element {
    rsx! {
        SEOHead {
            data: SEOProp {
                title: "About".into(),
                description: "Best of RS is a curated Rust ecosystem tracker that follows stars, forks, contributors, issues, and trend snapshots to help users and maintainers understand project momentum.".into(),
                keywords: "best of rs, about best of rs, rust ecosystem, rust open source tracking, rust project trends, github rust metrics".into(),
                canonical_url: "/about".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        GridSlashTransition { }
        GridWrapper {
            bg_class: "opacity-76",
            padding: GridPadding::None,
            background: GridBackground {
                pattern: GridPattern::Dot,
                gradient: GradientDirection::ToTop,
            },
            section { class: "min-h-screen px-4 py-10 md:px-30 md:py-20",
                CommonMarkdown {
                    src: ABOUT_MD.to_string(),
                    class: Some("about-markdown max-w-5xl".to_string()),
                }
            }
        }
        GridSlashTransition {  }
    }
}
