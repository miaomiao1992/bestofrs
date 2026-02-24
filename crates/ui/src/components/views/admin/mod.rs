mod ingest_daily_snapshots_control;
mod project_management;
use dioxus::prelude::*;
use crate::components::common::IOCell;
pub use ingest_daily_snapshots_control::IngestDailySnapshotsControl;
pub use project_management::ProjectManagement;

#[component]
pub fn AdminProjects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { ProjectManagement {} }
    }
}

#[component]
pub fn AdminTags() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { IngestDailySnapshotsControl {} }
    }
}
