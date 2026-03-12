use dioxus::prelude::*;
use crate::components::views::about::About;

#[component]
pub fn AboutView() -> Element {
    rsx! { About {} }
}
