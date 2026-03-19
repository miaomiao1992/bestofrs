pub use crate::components::providers::{UserContext, UserState};
use crate::{
    components::providers::{ConfigContext, ConfigProvider, UserProvider},
    components::{toast::ToastProvider, ScrollProgress, ScrollToTop},
    root::theme::theme_seed,
    root::Route,
    IO::auth::me,
};
use dioxus::prelude::*;

#[component]
pub fn RootLayout() -> Element {
    // use `try_use_context` to avoid `client side` error,
    // get it from `serve side` and init ConfigProvider for inner component
    let config = try_use_context::<ConfigContext>().unwrap_or_default();

    let me_fut = use_server_future(me)?;

    let user_state = match me_fut() {
        Some(Ok(Some(me))) => UserState::User(me),
        Some(Ok(None)) => UserState::Guest,
        Some(Err(err)) => UserState::Error(err.to_string()),
        None => UserState::Loading,
    };

    use_effect(move || {
        theme_seed();
    });

    rsx! {
        ToastProvider {
            ScrollProgress {}
            ScrollToTop {}
            ConfigProvider { config: config,
                UserProvider { state: user_state,
                    Outlet::<Route> {}
                }
            }
        }
    }
}
