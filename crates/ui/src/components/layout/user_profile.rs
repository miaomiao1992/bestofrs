use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::root::layouts::{UserContext, UserState};

#[component]
pub fn UserProfile() -> Element {
    let user_state = use_context::<UserContext>();

    rsx! {
        match user_state() {
            UserState::User(me) => {
                let fallback = me
                    .login
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase().to_string())
                    .unwrap_or_else(|| "?".to_string());

                rsx! {
                    Avatar { size: AvatarImageSize::Small,
                        if let Some(url) = me.avatar_url {
                            AvatarImage { src: url, alt: me.login }
                        }
                        AvatarFallback { "{fallback}" }
                    }
                }
            }
            _ => rsx! {},
        }
    }
}
