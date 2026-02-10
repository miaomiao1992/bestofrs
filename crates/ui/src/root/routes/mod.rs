mod admin;
mod home;
mod login;
mod repo;
mod tag;

use super::layouts::{AdminLayout, RootLayout};
use dioxus::prelude::*;

use admin::AdminView;
use home::HomeView;
use login::LoginView;
use repo::{RepoDetailView, RepoListView};
use tag::TagListView;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        HomeView {},
        #[nest("/repo")]
            #[route("/")]
            RepoListView {},
            #[route("/:owner/:name")]
            RepoDetailView { owner: String, name: String },
        #[end_nest]
        #[route("/tag")]
        TagListView {},
        #[layout(AdminLayout)]
            #[route("/admin")]
            AdminView {},
        #[end_layout]
        #[route("/login")]
        LoginView {},
}
