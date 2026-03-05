mod job;
mod projects;
mod tags;
use dioxus::prelude::*;
use crate::components::common::IOCell;
pub use job::Job;
pub use projects::Projects;
pub use tags::Tags;

#[component]
pub fn AdminProjects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { Projects {} }
    }
}

#[component]
pub fn AdminTags() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { Tags {} }
    }
}

#[component]
pub fn AdminJob() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { Job {} }
    }
}
