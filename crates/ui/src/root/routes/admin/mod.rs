use dioxus::prelude::*;

use crate::components::views::admin::{Job, Projects, Tags};

#[component]
pub fn AdminProjectsView() -> Element {
    rsx! { Projects {} }
}

#[component]
pub fn AdminTagsView() -> Element {
    rsx! { Tags {} }
}

#[component]
pub fn AdminJobView() -> Element {
    rsx! { Job {} }
}
