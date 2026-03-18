use dioxus::prelude::*;

use crate::types::tags::TagListItemDto;

#[derive(Clone, Copy)]
pub(super) struct TagsContext {
    pub(super) refresh: Signal<u32>,
    pub(super) search_key: Signal<String>,
}

#[derive(Clone, PartialEq)]
pub(super) enum TagPanelMode {
    Add,
    Edit(TagListItemDto),
}
