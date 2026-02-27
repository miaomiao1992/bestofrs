use dioxus::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};

pub fn build_repo_avatar_urls(
    repo_id: &str,
    avatar_url: Option<String>,
    homepage_url: Option<String>,
) -> Vec<String> {
    let mut urls = Vec::new();
    if let Some(homepage_url) = homepage_url {
        let trimmed = homepage_url.trim().trim_end_matches('/');
        if !trimmed.is_empty() {
            urls.push(format!("{trimmed}/favicon.ico"));
        }
    }
    if let Some(avatar_url) = avatar_url {
        if !urls.contains(&avatar_url) {
            urls.push(avatar_url);
        }
    }
    if let Some((owner, _)) = repo_id.split_once('/') {
        let owner_avatar = format!("https://github.com/{owner}.png");
        if !urls.contains(&owner_avatar) {
            urls.push(owner_avatar);
        }
    }
    let fallback = "https://github.com/github.png".to_string();
    if !urls.contains(&fallback) {
        urls.push(fallback);
    }
    urls
}

#[derive(Props, Clone, PartialEq)]
pub struct RepoAvatarProps {
    pub repo_id: String,
    pub avatar_urls: Vec<String>,
    #[props(default = AvatarImageSize::Large)]
    pub size: AvatarImageSize,
    #[props(default = "border border-primary-6 bg-primary".to_string())]
    pub class: String,
    #[props(default = "flex h-16 w-16 items-center justify-center border border-primary-6 bg-primary-2 font-bold text-secondary-4".to_string())]
    pub fallback_class: String,
}

#[component]
pub fn RepoAvatar(props: RepoAvatarProps) -> Element {
    let avatar_urls_for_error = props.avatar_urls.clone();
    let mut avatar_index = use_signal(|| 0usize);
    let avatar_fallback = props
        .repo_id
        .split('/')
        .nth(1)
        .unwrap_or(props.repo_id.as_str())
        .chars()
        .next()
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    if let Some(src) = props.avatar_urls.get(avatar_index()).cloned() {
        rsx! {
            Avatar {
                key: "{src}",
                class: "{props.class}",
                size: props.size,
                on_error: move |_| {
                    let next = avatar_index() + 1;
                    if next < avatar_urls_for_error.len() {
                        avatar_index.set(next);
                    } else {
                        avatar_index.set(usize::MAX);
                    }
                },
                AvatarImage {
                    src: src,
                    alt: "{props.repo_id} avatar",
                }
                AvatarFallback { "{avatar_fallback}" }
            }
        }
    } else {
        rsx! {
            div { class: "{props.fallback_class}",
                "{avatar_fallback}"
            }
        }
    }
}
