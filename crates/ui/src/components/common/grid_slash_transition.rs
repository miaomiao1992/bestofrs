use dioxus::prelude::*;

use super::{GradientDirection, GridBackground, GridPadding, GridPattern, GridType, GridWrapper};

#[component]
pub fn GridSlashTransition() -> Element {
    rsx! {
        GridWrapper {
            class: "h-4 -z-0",
            padding: GridPadding::None,
            is_dot_on: true,
            grid_type: GridType::Default,
            background: GridBackground {
                pattern: GridPattern::Slash,
                gradient: GradientDirection::None,
            },
            div { class: "h-full w-full" }
        }
    }
}
