use super::layouts::{AdminLayout, RootLayout};
use super::views::{Admin, Home, Login, RepoDetail, RepoList, TagList};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Home {},
        #[route("/repo")]
        RepoList {},
        #[route("/tag")]
        TagList {},
        #[layout(AdminLayout)]
            #[route("/admin")]
            Admin {},
        #[end_layout]
        #[route("/login")]
        Login {},
        #[route("/repo/:owner/:name")]
        RepoDetail { owner: String, name: String },
}
