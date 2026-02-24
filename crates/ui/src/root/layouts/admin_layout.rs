use dioxus::prelude::*;

use crate::components::AdminStateHint;
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider,
    SidebarRail, SidebarTrigger, SidebarVariant,
};
use crate::root::layouts::{UserContext, UserState};
use crate::root::Route;

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    let navigator_for_projects = navigator.clone();
    let navigator_for_tags = navigator.clone();
    let current_route = use_route::<Route>();
    let mut redirected = use_signal(|| false);
    let user_state = use_context::<UserContext>();
    let is_admin = matches!(user_state(), UserState::User(me) if me.role == "Admin");
    let is_projects = matches!(current_route, Route::AdminProjectsView {});
    let is_tags = matches!(current_route, Route::AdminTagsView {});

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
            div { class: "min-h-[calc(100svh-4rem)]",
                SidebarProvider {
                    Sidebar {
                        variant: SidebarVariant::Sidebar,
                        collapsible: SidebarCollapsible::Icon,
                        SidebarHeader {
                            div { class: "rounded-md border border-primary-6 bg-primary-2 px-3 py-2",
                                div { class: "font-mono text-[11px] font-semibold tracking-widest text-secondary-5", "ADMIN / CONTROL" }
                                div { class: "mt-1 text-sm font-semibold text-secondary-3", "BestOfRust Console" }
                            }
                        }
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupLabel { "Management" }
                                SidebarMenu {
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: is_projects,
                                            as: move |attributes: Vec<Attribute>| rsx! {
                                                button {
                                                    onclick: move |_| {
                                                        let _ = navigator_for_projects.push(Route::AdminProjectsView {});
                                                    },
                                                    ..attributes,
                                                    span { "Project 管理" }
                                                }
                                            },
                                        }
                                    }
                                    SidebarMenuItem {
                                        SidebarMenuButton {
                                            is_active: is_tags,
                                            as: move |attributes: Vec<Attribute>| rsx! {
                                                button {
                                                    onclick: move |_| {
                                                        let _ = navigator_for_tags.push(Route::AdminTagsView {});
                                                    },
                                                    ..attributes,
                                                    span { "Tags / Jobs 管理" }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                        SidebarFooter {
                            div { class: "border-t border-dashed border-primary-6 px-2 pt-2 text-xs font-mono text-secondary-5",
                                "role: admin"
                            }
                        }
                        SidebarRail {}
                    }
                    SidebarInset {
                        header { class: "flex h-14 shrink-0 items-center gap-3 border-b border-primary-6 bg-primary-1 px-4",
                            SidebarTrigger {}
                            Separator { height: "1rem", horizontal: false }
                            div { class: "min-w-0",
                                div { class: "font-mono text-[11px] tracking-widest text-secondary-5", "ADMIN PANEL" }
                                h1 { class: "truncate text-sm font-semibold text-secondary-3 md:text-base",
                                    if is_projects { "Project Management" } else { "Tags / Jobs Management" }
                                }
                            }
                        }
                        div { class: "min-h-0 flex-1 overflow-y-auto overflow-x-hidden p-4 md:p-6",
                            SuspenseBoundary {
                                fallback: move |_: SuspenseContext| {
                                    rsx! { AdminStateHint { message: "Loading...".to_string() } }
                                },
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
            }
        } else if matches!(user_state(), UserState::Loading) {
            AdminStateHint { message: "Loading...".to_string() }
        } else {
            AdminStateHint { message: "Redirecting...".to_string() }
        }
    }
}
