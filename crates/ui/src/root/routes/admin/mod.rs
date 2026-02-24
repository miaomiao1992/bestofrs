use dioxus::prelude::*;

use crate::components::views::admin::{AdminProjects, AdminTags};

#[component]
pub fn AdminProjectsView() -> Element {
    rsx! { AdminProjects {} }
}

#[component]
pub fn AdminTagsView() -> Element {
    rsx! { AdminTags {} }
}
