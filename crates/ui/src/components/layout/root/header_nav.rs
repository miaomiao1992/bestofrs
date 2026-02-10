use dioxus::prelude::*;

use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn HeaderNav() -> Element {
    let user_state = use_context::<UserContext>();
    let show_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");

    rsx! {
        nav { class: "flex items-center gap-4 text-sm",
            Link { class: "text-secondary-5 hover:text-secondary-4 hover:underline", to: Route::HomeView {}, "Home" }
            Link { class: "text-secondary-5 hover:text-secondary-4 hover:underline", to: Route::RepoListView {}, "Repo" }
            Link { class: "text-secondary-5 hover:text-secondary-4 hover:underline", to: Route::TagListView {}, "Tag" }
            if show_admin {
                Link { class: "text-secondary-5 hover:text-secondary-4 hover:underline", to: Route::AdminView {}, "Admin" }
            }
        }
    }
}
