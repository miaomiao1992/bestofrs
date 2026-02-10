use dioxus::prelude::*;

use crate::components::AdminStateHint;
use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    let mut redirected = use_signal(|| false);
    let user_state = use_context::<UserContext>();
    let is_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");

    use_effect(move || {
        if redirected() {
            return;
        }

        let unauthorized = match user_state() {
            UserState::User(me) => me.role != "Admin",
            UserState::Loading => false,
            UserState::Guest | UserState::Error(_) => true,
        };

        if unauthorized {
            redirected.set(true);
            navigator.replace(Route::HomeView {});
        }
    });

    rsx! {
        if is_admin {
            Outlet::<Route> {}
        } else if matches!(user_state(), UserState::Loading) {
            AdminStateHint { message: "Loading...".to_string() }
        } else {
            AdminStateHint { message: "Redirecting...".to_string() }
        }
    }
}
