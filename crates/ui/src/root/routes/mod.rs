mod admin;
mod home;
mod login;
mod repo;
mod tag;

use super::layouts::{AdminLayout, RootLayout};
use dioxus::prelude::*;
use admin::{AdminProjectsView, AdminTagsView};
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
            #[nest("/admin")]
                #[redirect("/", || Route::AdminProjectsView {})]
                #[route("/projects")]
                AdminProjectsView {},
                #[route("/tags")]
                AdminTagsView {},
            #[end_nest]
        #[end_layout]
        #[route("/login")]
        LoginView {},
}
