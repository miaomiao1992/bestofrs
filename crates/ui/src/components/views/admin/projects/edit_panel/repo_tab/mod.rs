pub(crate) mod skeleton;

use dioxus::prelude::*;
use crate::components::icons::{PlusIcon, TrashIcon};
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;

use crate::IO::repos::{get_repo, list_tags_with_meta, replace_repo_tags};
use crate::types::tags::TagDto;

use super::super::context::{ProjectPanelMode, ProjectsContext};

fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

#[derive(Clone, PartialEq, Default)]
struct RepoEditorState {
    pending: bool,
    message: Option<String>,
    current_tags: Vec<TagDto>,
    selected_tags: Vec<TagDto>,
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct RepoTabProps {
    pub mode: ProjectPanelMode,
    pub busy: Signal<bool>,
}

#[component]
pub(super) fn RepoTab(props: RepoTabProps) -> Element {
    let mut refresh = use_context::<ProjectsContext>().refresh;
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let mut state = use_signal(RepoEditorState::default);
    let mut mode_snapshot = use_signal(|| Option::<ProjectPanelMode>::None);
    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        state.set(RepoEditorState::default());
    }

    let state_value = state();
    let mut busy = props.busy;
    if busy() != state_value.pending {
        busy.set(state_value.pending);
    }

    let all_tags = match tags_page() {
        Some(Ok(page)) => page
            .items
            .into_iter()
            .map(|tag| TagDto {
                label: tag.label,
                value: tag.value,
                description: tag.description,
                repos_total: Some(tag.repos_total),
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    match props.mode.clone() {
        ProjectPanelMode::Add => rsx! {
            div { class: "rounded-md border border-dashed border-primary-6 bg-primary px-3 py-6 text-center text-sm text-secondary-5",
                "请先创建 project 并进入 Edit 后再编辑关联 repo tags"
            }
        },
        ProjectPanelMode::Edit(project) => rsx! {
            div { class: "space-y-3 border border-primary-6 bg-primary p-3",
                div { class: "text-xs font-mono text-secondary-5", "REPO TAGS EDITOR" }
                div { class: "text-xs text-secondary-5", "关联 Repo: {project.repo_id}" }
                Button {
                    class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                    disabled: state_value.pending,
                    onclick: {
                        let target_repo_id = project.repo_id.clone();
                        move |_| {
                            let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                state.with_mut(|s| {
                                    s.message = Some(format!("无效 repo id: {target_repo_id}"));
                                    s.current_tags.clear();
                                    s.selected_tags.clear();
                                });
                                return;
                            };

                            state.with_mut(|s| {
                                s.pending = true;
                                s.message = None;
                            });

                            spawn(async move {
                                match get_repo(owner, name).await {
                                    Ok(Some(repo_detail)) => {
                                        state.with_mut(|s| {
                                            s.current_tags = repo_detail.tags;
                                            s.selected_tags.clear();
                                            s.message = Some("已加载关联 repo tags".to_string());
                                        });
                                    }
                                    Ok(None) => {
                                        state.with_mut(|s| {
                                            s.current_tags.clear();
                                            s.selected_tags.clear();
                                            s.message = Some("未找到该 repo".to_string());
                                        });
                                    }
                                    Err(err) => {
                                        state.with_mut(|s| {
                                            s.current_tags.clear();
                                            s.selected_tags.clear();
                                            s.message = Some(err.to_string());
                                        });
                                    }
                                }
                                state.with_mut(|s| s.pending = false);
                            });
                        }
                    },
                    "加载当前 Repo Tags"
                }
                div { class: "text-xs font-semibold text-secondary-5", "当前已绑定 Tags" }
                if state_value.current_tags.is_empty() {
                    div { class: "text-xs text-secondary-5", "（空）" }
                } else {
                    div { class: "flex flex-wrap gap-2",
                        for tag in state_value.current_tags.clone() {
                            span { key: "cur-{tag.label}:{tag.value}", class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs",
                                "{tag.label}:{tag.value}"
                            }
                        }
                    }
                }
                div { class: "text-xs font-semibold text-secondary-5", "选择 tags 更新到该 Repo" }
                div { class: "max-h-[220px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                    for tag in all_tags.clone() {
                        label { key: "repo-editor-{tag.label}:{tag.value}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                            Input {
                                r#type: "checkbox",
                                checked: state_value.selected_tags.iter().any(|x| x.label == tag.label && x.value == tag.value),
                                onchange: {
                                    let target_tag = tag.clone();
                                    move |_| {
                                        state.with_mut(|s| {
                                            if let Some(index) = s.selected_tags.iter().position(|x| x == &target_tag) {
                                                s.selected_tags.remove(index);
                                            } else {
                                                s.selected_tags.push(target_tag.clone());
                                            }
                                        });
                                    }
                                },
                            }
                            span { class: "text-xs", "{tag.label}:{tag.value}" }
                        }
                    }
                }
                div { class: "flex flex-wrap gap-2",
                    Button {
                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                        disabled: state_value.pending,
                        onclick: {
                            let target_repo_id = project.repo_id.clone();
                            move |_| {
                                let picked_tags = state().selected_tags;
                                if picked_tags.is_empty() {
                                    state.with_mut(|s| s.message = Some("请先选择 tag".to_string()));
                                    return;
                                }
                                let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                    state.with_mut(|s| s.message = Some(format!("无效 repo id: {target_repo_id}")));
                                    return;
                                };
                                let mut next_tags = state().current_tags;
                                for tag in picked_tags {
                                    if !next_tags.contains(&tag) {
                                        next_tags.push(tag);
                                    }
                                }
                                state.with_mut(|s| {
                                    s.pending = true;
                                    s.message = None;
                                });
                                spawn(async move {
                                    match replace_repo_tags(owner, name, next_tags.clone()).await {
                                        Ok(()) => {
                                            state.with_mut(|s| {
                                                s.current_tags = next_tags;
                                                s.message = Some("Add 完成".to_string());
                                            });
                                        }
                                        Err(err) => state.with_mut(|s| s.message = Some(err.to_string())),
                                    }
                                    state.with_mut(|s| s.pending = false);
                                });
                            }
                        },
                        span { class: "inline-flex items-center gap-1",
                            PlusIcon { width: 14, height: 14 }
                            "Add"
                        }
                    }
                    Button {
                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                        disabled: state_value.pending,
                        onclick: {
                            let target_repo_id = project.repo_id.clone();
                            move |_| {
                                let picked_tags = state().selected_tags;
                                if picked_tags.is_empty() {
                                    state.with_mut(|s| s.message = Some("请先选择 tag".to_string()));
                                    return;
                                }
                                let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                    state.with_mut(|s| s.message = Some(format!("无效 repo id: {target_repo_id}")));
                                    return;
                                };
                                let mut next_tags = state().current_tags;
                                next_tags.retain(|tag| !picked_tags.contains(tag));
                                state.with_mut(|s| {
                                    s.pending = true;
                                    s.message = None;
                                });
                                spawn(async move {
                                    match replace_repo_tags(owner, name, next_tags.clone()).await {
                                        Ok(()) => {
                                            state.with_mut(|s| {
                                                s.current_tags = next_tags;
                                                s.message = Some("Remove 完成".to_string());
                                            });
                                        }
                                        Err(err) => state.with_mut(|s| s.message = Some(err.to_string())),
                                    }
                                    state.with_mut(|s| s.pending = false);
                                });
                            }
                        },
                        span { class: "inline-flex items-center gap-1",
                            TrashIcon { width: 14, height: 14 }
                            "Remove"
                        }
                    }
                }
                if state_value.pending {
                    div { class: "text-xs text-secondary-5", "处理中..." }
                }
                if let Some(msg) = state_value.message {
                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                }
            }
        },
    }
}
