use dioxus::prelude::*;

use crate::components::views::home::Home;

#[component]
pub fn HomeView() -> Element {
    rsx! { Home {} }
}
