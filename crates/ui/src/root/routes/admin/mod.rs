use dioxus::prelude::*;

use crate::components::views::admin::Admin;

#[component]
pub fn AdminView() -> Element {
    rsx! { Admin {} }
}
