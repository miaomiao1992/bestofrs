use dioxus::prelude::*;
use crate::types::projects::ProjectDto;

#[derive(Clone, Copy)]
pub(super) struct ProjectsContext {
    pub(super) refresh: Signal<u32>,
    pub(super) search_key: Signal<String>,
}

#[derive(Clone, PartialEq)]
pub(super) enum ProjectPanelMode {
    Add,
    Edit(ProjectDto),
}
