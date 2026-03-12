use dioxus::prelude::*;

use crate::components::common::{
    CommonMarkdown, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper,
};

const ABOUT_MD: &str = include_str!("./about.md");

#[component]
pub fn About() -> Element {
    rsx! {
        GridSlashTransition { }
        GridWrapper {
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Grid,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "min-h-screen",
                CommonMarkdown {
                    src: ABOUT_MD.to_string(),
                    class: Some("max-w-4xl p-6 md:p-10".to_string()),
                }
            }
        }
        GridSlashTransition {  }
    }
}
