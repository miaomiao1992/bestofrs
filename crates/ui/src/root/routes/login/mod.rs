use dioxus::prelude::*;

use crate::components::views::login::Login;

#[component]
pub fn LoginView() -> Element {
    rsx! { Login {} }
}
