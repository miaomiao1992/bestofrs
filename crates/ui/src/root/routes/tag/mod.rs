use dioxus::prelude::*;

use crate::components::views::tag::TagList;

#[component]
pub fn TagListView() -> Element {
    rsx! { TagList {} }
}
