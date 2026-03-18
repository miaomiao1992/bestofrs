mod project_tab;
mod repo_tab;

use dioxus::prelude::*;

use crate::components::common::IOCell;
use crate::components::icons::XIcon;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::ui::button::{Button, ButtonVariant};

use super::context::ProjectPanelMode;
use project_tab::skeleton::ProjectTabSkeleton;
use project_tab::ProjectTab;
use repo_tab::skeleton::RepoTabSkeleton;
use repo_tab::RepoTab;

#[derive(Props, Clone, PartialEq)]
pub(super) struct EditPanelProps {
    pub mode: ProjectPanelMode,
    pub on_close: Callback<()>,
}

#[component]
pub(super) fn EditPanel(props: EditPanelProps) -> Element {
    let mut panel_tab = use_signal(|| Some("project".to_string()));
    let panel_tab_read: ReadSignal<Option<String>> = panel_tab.into();
    let mut mode_snapshot = use_signal(|| Option::<ProjectPanelMode>::None);
    let mut project_tab_busy = use_signal(|| false);
    let mut repo_tab_busy = use_signal(|| false);

    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        panel_tab.set(Some("project".to_string()));
    }

    rsx! {
        div { class: "min-w-0 flex-1 space-y-3",
            section { class: "space-y-3 border border-primary-6 bg-primary-1 p-4",
                div { class: "flex items-center justify-between",
                    div { class: "text-sm font-semibold",
                        if matches!(props.mode, ProjectPanelMode::Add) { "Add Project" } else { "Edit Project" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "button rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                        disabled: project_tab_busy() || repo_tab_busy(),
                        onclick: move |_: MouseEvent| props.on_close.call(()),
                        XIcon { width: 16, height: 16 }
                    }
                }
                Tabs {
                    class: "space-y-3".to_string(),
                    value: panel_tab_read,
                    default_value: "project".to_string(),
                    on_value_change: move |value| panel_tab.set(Some(value)),
                    TabList {
                        TabTrigger { value: "project".to_string(), index: 0usize, "project" }
                        TabTrigger { value: "repo".to_string(), index: 1usize, "repo" }
                    }
                    TabContent {
                        value: "project".to_string(),
                        index: 0usize,
                        IOCell {
                            loading_fallback: rsx! { ProjectTabSkeleton {} },
                            ProjectTab {
                                mode: props.mode.clone(),
                                busy: project_tab_busy,
                            }
                        }
                    }
                    TabContent {
                        value: "repo".to_string(),
                        index: 1usize,
                        IOCell {
                            loading_fallback: rsx! { RepoTabSkeleton {} },
                            RepoTab {
                                mode: props.mode.clone(),
                                busy: repo_tab_busy,
                            }
                        }
                    }
                }
            }
        }
    }
}
