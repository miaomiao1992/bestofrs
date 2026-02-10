use dioxus::prelude::*;

use crate::components::views::repo::RepoListContent;

#[component]
pub fn Home() -> Element {
    rsx! { RepoListContent { title: "Home".to_string(), subtitle: "Repo list".to_string() } }
}
