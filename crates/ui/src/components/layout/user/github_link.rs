use crate::components::icons::GithubIcon;
use dioxus::prelude::*;

const BESTOFRS_GITHUB_URL: &str = "https://github.com/zhiyanzhaijie/bestofrs";

#[component]
pub fn GithubLink() -> Element {
    rsx! {
        Link {
            class: "inline-flex h-[1.6rem] w-[1.6rem] items-center justify-center rounded-full p-0 text-secondary-5 shadow-none transition-colors hover:text-secondary-4 hover:cursor-pointer",
            to: BESTOFRS_GITHUB_URL,
            new_tab: true,
            rel: "noopener noreferrer",
            aria_label: "Bestofrs GitHub",
            title: "Bestofrs GitHub",
            GithubIcon { width: 18, height: 18 }
        }
    }
}
