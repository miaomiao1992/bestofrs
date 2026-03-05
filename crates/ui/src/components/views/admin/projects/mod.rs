use dioxus::prelude::*;

use crate::components::common::CommonPagination;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::IO::projects::{
    import_projects, import_projects_json, list_projects, remove_project, search_projects,
    update_projects,
};
use crate::IO::repos::{get_repo, list_tags_with_meta, replace_repo_tags};
use crate::types::projects::{ProjectDto, ProjectImportItem};
use crate::types::tags::TagDto;
use app::prelude::{Page, Pagination};

fn optional_text(value: String) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn dedup_values(values: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for value in values {
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

fn empty_projects_page(page: Pagination) -> Page<ProjectDto> {
    page.to_page(Vec::new(), 0)
}

fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

fn paginate_items<T: Clone>(items: &[T], current_page: u32, page_size: usize) -> (Vec<T>, u32) {
    if items.is_empty() {
        return (Vec::new(), 1);
    }
    let total_pages = items.len().div_ceil(page_size) as u32;
    let current_page = current_page.clamp(1, total_pages);
    let start = ((current_page - 1) as usize) * page_size;
    let end = (start + page_size).min(items.len());
    (items[start..end].to_vec(), total_pages)
}

#[derive(Clone, PartialEq)]
enum ProjectPanelMode {
    Add,
    Edit(ProjectDto),
}

#[component]
pub fn Projects() -> Element {
    let mut refresh = use_signal(|| 0u32);
    let projects = use_server_future(move || {
        let _ = refresh();
        list_projects(Pagination {
            limit: Some(500),
            offset: Some(0),
        })
    })?;
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let search_page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };
    let mut search_key = use_signal(String::new);
    let mut table_page = use_signal(|| 1u32);
    let page_size = 20usize;
    let mut search = use_action({
        let page = search_page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_projects_page(page));
            }
            search_projects(key, page).await
        }
    });

    let mut panel_mode = use_signal(|| Option::<ProjectPanelMode>::None);
    let mut panel_tab = use_signal(|| Some("project".to_string()));
    let panel_tab_read: ReadSignal<Option<String>> = panel_tab.into();
    let mut create_or_edit_pending = use_signal(|| false);
    let mut remove_pending = use_signal(|| false);
    let mut panel_message = use_signal(|| Option::<String>::None);
    let mut table_message = use_signal(|| Option::<String>::None);
    let mut repo_editor_pending = use_signal(|| false);
    let mut repo_editor_message = use_signal(|| Option::<String>::None);
    let mut repo_editor_current_tags = use_signal(Vec::<TagDto>::new);
    let mut repo_editor_selected_tags = use_signal(Vec::<TagDto>::new);

    let mut form_repo_id = use_signal(String::new);
    let mut form_name = use_signal(String::new);
    let mut form_slug = use_signal(String::new);
    let mut form_description = use_signal(String::new);
    let mut form_url = use_signal(String::new);
    let mut form_avatar_url = use_signal(String::new);
    let mut form_status = use_signal(String::new);
    let mut form_logo = use_signal(String::new);
    let mut form_twitter = use_signal(String::new);
    let mut form_tag_query = use_signal(String::new);
    let mut form_selected_tag_values = use_signal(Vec::<String>::new);
    let mut json_import_pending = use_signal(|| false);
    let mut json_import_message = use_signal(|| Option::<String>::None);
    let mut json_file_name = use_signal(String::new);
    let mut json_file_text = use_signal(String::new);

    let panel_open = panel_mode().is_some();
    let search_on = !search_key().trim().is_empty();

    let table_items = if search_on {
        match search.value() {
            Some(Ok(page)) => page().items.clone(),
            _ => Vec::new(),
        }
    } else {
        match projects() {
            Some(Ok(page)) => page.items.clone(),
            _ => Vec::new(),
        }
    };

    let (paged_items, total_pages) = paginate_items(&table_items, table_page(), page_size);
    let total_items = table_items.len() as u32;
    let all_tags = match tags_page() {
        Some(Ok(page)) => page
            .items
            .into_iter()
            .map(|tag| TagDto {
                label: tag.label,
                value: tag.value,
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };
    let tag_filter_key = form_tag_query().trim().to_lowercase();
    let filtered_form_tags = all_tags
        .iter()
        .filter(|tag| {
            if tag_filter_key.is_empty() {
                return true;
            }
            let text = format!(
                "{}:{}",
                tag.label.to_lowercase(),
                tag.value.to_lowercase()
            );
            text.contains(&tag_filter_key)
        })
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "PROJECTS / MANAGEMENT" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Project 管理" }
                p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                    "右侧面板打开时，左侧保留 reponame 与 edit，便于快速切换。"
                }
            }
            div { class: "flex flex-col gap-2 md:flex-row",
                input {
                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                    placeholder: "搜索 repo_id / name / slug / description",
                    value: search_key,
                    oninput: move |e| {
                        *search_key.write() = e.value();
                        table_page.set(1);
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            table_page.set(1);
                            search.call(search_key());
                        }
                    },
                }
                button {
                    class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                    onclick: move |_| {
                        table_page.set(1);
                        search.call(search_key());
                    },
                    "搜索"
                }
                button {
                    class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                    onclick: move |_| {
                        search_key.set(String::new());
                        table_page.set(1);
                    },
                    "清空搜索"
                }
                button {
                    class: "rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90",
                    onclick: move |_| {
                        panel_mode.set(Some(ProjectPanelMode::Add));
                        panel_tab.set(Some("project".to_string()));
                        panel_message.set(None);
                        repo_editor_message.set(None);
                        repo_editor_current_tags.set(Vec::new());
                        repo_editor_selected_tags.set(Vec::new());
                        form_repo_id.set(String::new());
                        form_name.set(String::new());
                        form_slug.set(String::new());
                        form_description.set(String::new());
                        form_url.set(String::new());
                        form_avatar_url.set(String::new());
                        form_status.set(String::new());
                        form_logo.set(String::new());
                        form_twitter.set(String::new());
                        form_tag_query.set(String::new());
                        form_selected_tag_values.set(Vec::new());
                    },
                    "Add Project"
                }
            }
            if total_pages > 1 {
                div { class: "flex justify-center",
                    CommonPagination {
                        current_page: table_page(),
                        total_pages,
                        on_page_change: move |p| table_page.set(p),
                    }
                }
            }

            div { class: "flex items-start gap-4 overflow-x-auto",
                div { class: if panel_open { "w-[420px] shrink-0 space-y-3" } else { "min-w-0 flex-1 space-y-3" },

                    div { class: "overflow-auto rounded-md border border-primary-6 bg-primary-1",
                        table { class: "min-w-full text-sm",
                            thead { class: "border-b border-primary-6 bg-primary",
                                tr {
                                    th { class: "px-3 py-2 text-left font-medium text-secondary-5", "reponame" }
                                    if !panel_open {
                                        th { class: "px-3 py-2 text-left font-medium text-secondary-5", "repo_id" }
                                        th { class: "px-3 py-2 text-left font-medium text-secondary-5", "slug" }
                                    }
                                    th { class: "px-3 py-2 text-right font-medium text-secondary-5", "actions" }
                                }
                            }
                            tbody {
                                if search_on && search.value().is_none() {
                                    tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "4" }, "输入关键字后执行搜索" } }
                                } else if search_on {
                                    if let Some(Err(err)) = search.value() {
                                        tr { td { class: "px-3 py-6 text-center text-primary-error", colspan: if panel_open { "2" } else { "4" }, "{err}" } }
                                    } else if paged_items.is_empty() {
                                        tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "4" }, "无搜索结果" } }
                                    } else {
                                        for p in paged_items {
                                            tr { key: "search-{p.id}", class: "border-b border-primary-6 last:border-b-0",
                                                td { class: "px-3 py-2", "{p.name}" }
                                                if !panel_open {
                                                    td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{p.repo_id}" }
                                                    td { class: "px-3 py-2 text-secondary-5", "{p.slug}" }
                                                }
                                                td { class: "px-3 py-2",
                                                    div { class: "flex justify-end gap-2",
                                                        button {
                                                            class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                            disabled: remove_pending() || create_or_edit_pending(),
                                                            onclick: {
                                                                let project = p.clone();
                                                                move |_| {
                                                                    panel_mode.set(Some(ProjectPanelMode::Edit(project.clone())));
                                                                    panel_tab.set(Some("project".to_string()));
                                                                    panel_message.set(None);
                                                                    repo_editor_message.set(None);
                                                                    repo_editor_current_tags.set(Vec::new());
                                                                    repo_editor_selected_tags.set(Vec::new());
                                                                    form_repo_id.set(project.repo_id.clone());
                                                                    form_name.set(project.name.clone());
                                                                    form_slug.set(project.slug.clone());
                                                                    form_description.set(project.description.clone());
                                                                    form_url.set(project.url.clone().unwrap_or_default());
                                                                    form_avatar_url.set(project.avatar_url.clone().unwrap_or_default());
                                                                    form_status.set(project.status.clone().unwrap_or_default());
                                                                    form_logo.set(project.logo.clone().unwrap_or_default());
                                                                    form_twitter.set(project.twitter.clone().unwrap_or_default());
                                                                    form_tag_query.set(String::new());
                                                                                form_selected_tag_values.set(Vec::new());
                                                                }
                                                            },
                                                            "Edit"
                                                        }
                                                        if !panel_open {
                                                            button {
                                                                class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                                disabled: remove_pending() || create_or_edit_pending(),
                                                                    onclick: {
                                                                        let repo_id = p.repo_id.clone();
                                                                        move |_| {
                                                                    *remove_pending.write() = true;
                                                                    *table_message.write() = None;
                                                                        let target_repo_id = repo_id.clone();
                                                                    spawn(async move {
                                                                            match remove_project(target_repo_id.clone()).await {
                                                                            Ok(()) => {
                                                                                    *table_message.write() = Some(format!("已删除 project: {target_repo_id}"));
                                                                                refresh.with_mut(|v| *v += 1);
                                                                            }
                                                                            Err(err) => *table_message.write() = Some(err.to_string()),
                                                                        }
                                                                        *remove_pending.write() = false;
                                                                    });
                                                                    }
                                                                    },
                                                                "Delete"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    match projects() {
                                        Some(Err(err)) => rsx! {
                                            tr { td { class: "px-3 py-6 text-center text-primary-error", colspan: if panel_open { "2" } else { "4" }, "{err}" } }
                                        },
                                        Some(Ok(_)) => {
                                            if paged_items.is_empty() {
                                                rsx! {
                                                    tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "4" }, "暂无数据" } }
                                                }
                                            } else {
                                                rsx! {
                                                    for p in paged_items {
                                                        tr { key: "{p.id}", class: "border-b border-primary-6 last:border-b-0",
                                                            td { class: "px-3 py-2", "{p.name}" }
                                                            if !panel_open {
                                                                td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{p.repo_id}" }
                                                                td { class: "px-3 py-2 text-secondary-5", "{p.slug}" }
                                                            }
                                                            td { class: "px-3 py-2",
                                                                div { class: "flex justify-end gap-2",
                                                                    button {
                                                                        class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                                        disabled: remove_pending() || create_or_edit_pending(),
                                                                        onclick: {
                                                                            let project = p.clone();
                                                                            move |_| {
                                                                                panel_mode.set(Some(ProjectPanelMode::Edit(project.clone())));
                                                                                panel_tab.set(Some("project".to_string()));
                                                                                panel_message.set(None);
                                                                                repo_editor_message.set(None);
                                                                                repo_editor_current_tags.set(Vec::new());
                                                                                repo_editor_selected_tags.set(Vec::new());
                                                                                form_repo_id.set(project.repo_id.clone());
                                                                                form_name.set(project.name.clone());
                                                                                form_slug.set(project.slug.clone());
                                                                                form_description.set(project.description.clone());
                                                                                form_url.set(project.url.clone().unwrap_or_default());
                                                                                form_avatar_url.set(project.avatar_url.clone().unwrap_or_default());
                                                                                form_status.set(project.status.clone().unwrap_or_default());
                                                                                form_logo.set(project.logo.clone().unwrap_or_default());
                                                                                form_twitter.set(project.twitter.clone().unwrap_or_default());
                                                                                form_tag_query.set(String::new());
                                                                                form_selected_tag_values.set(Vec::new());
                                                                            }
                                                                        },
                                                                        "Edit"
                                                                    }
                                                                    if !panel_open {
                                                                        button {
                                                                            class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                                            disabled: remove_pending() || create_or_edit_pending(),
                                                                            onclick: {
                                                                                let repo_id = p.repo_id.clone();
                                                                                move |_| {
                                                                                *remove_pending.write() = true;
                                                                                *table_message.write() = None;
                                                                                let target_repo_id = repo_id.clone();
                                                                                spawn(async move {
                                                                                    match remove_project(target_repo_id.clone()).await {
                                                                                        Ok(()) => {
                                                                                            *table_message.write() = Some(format!("已删除 project: {target_repo_id}"));
                                                                                            refresh.with_mut(|v| *v += 1);
                                                                                        }
                                                                                        Err(err) => *table_message.write() = Some(err.to_string()),
                                                                                    }
                                                                                    *remove_pending.write() = false;
                                                                                });
                                                                            }
                                                                            },
                                                                            "Delete"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        None => rsx! {
                                            tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "4" }, "Loading..." } }
                                        },
                                    }
                                }
                            }
                        }
                    }

                    div { class: "text-xs text-secondary-5", "{total_items} items" }
                    if remove_pending() {
                        div { class: "text-xs text-secondary-5", "删除中..." }
                    }
                    if let Some(msg) = table_message() {
                        div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                    }
                }

                if let Some(mode) = panel_mode() {
                    div { class: "min-w-0 flex-1 space-y-3",
                        section { class: "space-y-3 border border-primary-6 bg-primary-1 p-4",
                            div { class: "flex items-center justify-between",
                                div { class: "text-sm font-semibold",
                                    if matches!(mode, ProjectPanelMode::Add) { "Add Project" } else { "Edit Project" }
                                }
                                button {
                                    class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                    disabled: create_or_edit_pending() || repo_editor_pending() || json_import_pending(),
                                    onclick: move |_| {
                                        panel_mode.set(None);
                                        panel_message.set(None);
                                        json_import_message.set(None);
                                        repo_editor_message.set(None);
                                    },
                                    "关闭"
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
                                    if let ProjectPanelMode::Edit(ref project) = mode {
                                        div { class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs text-secondary-5",
                                            "editing: {project.repo_id}"
                                        }
                                    }
                                    div { class: "space-y-2",
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "repo_id (owner/name) *",
                                            value: form_repo_id,
                                            disabled: matches!(mode, ProjectPanelMode::Edit(_)),
                                            oninput: move |e| *form_repo_id.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "name *",
                                            value: form_name,
                                            oninput: move |e| *form_name.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "slug *",
                                            value: form_slug,
                                            oninput: move |e| *form_slug.write() = e.value(),
                                        }
                                        textarea {
                                            class: "w-full min-h-[80px] rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "description",
                                            value: form_description,
                                            oninput: move |e| *form_description.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "url",
                                            value: form_url,
                                            oninput: move |e| *form_url.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "avatar_url",
                                            value: form_avatar_url,
                                            oninput: move |e| *form_avatar_url.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "status",
                                            value: form_status,
                                            oninput: move |e| *form_status.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "logo",
                                            value: form_logo,
                                            oninput: move |e| *form_logo.write() = e.value(),
                                        }
                                        input {
                                            class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                            placeholder: "twitter",
                                            value: form_twitter,
                                            oninput: move |e| *form_twitter.write() = e.value(),
                                        }
                                        if matches!(mode, ProjectPanelMode::Add) {
                                            div { class: "space-y-2 rounded-md border border-primary-6 bg-primary p-2",
                                                input {
                                                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                                    placeholder: "搜索 tags(label/value)",
                                                    value: form_tag_query,
                                                    oninput: move |e| *form_tag_query.write() = e.value(),
                                                }
                                                div { class: "flex items-center justify-between gap-2",
                                                    div { class: "text-xs text-secondary-5", "已选 {form_selected_tag_values().len()} 项" }
                                                    button {
                                                        class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                                                        onclick: move |_| form_selected_tag_values.set(Vec::new()),
                                                        "清空已选"
                                                    }
                                                }
                                                div { class: "max-h-[180px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                                                    for tag in filtered_form_tags.clone() {
                                                        label { key: "project-form-tag-{tag.label}:{tag.value}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                                            input {
                                                                r#type: "checkbox",
                                                                checked: form_selected_tag_values().contains(&tag.value),
                                                                onchange: {
                                                                    let value = tag.value.clone();
                                                                    move |_| {
                                                                        let mut selected = form_selected_tag_values();
                                                                        if let Some(index) = selected.iter().position(|x| x == &value) {
                                                                            selected.remove(index);
                                                                        } else {
                                                                            selected.push(value.clone());
                                                                        }
                                                                        form_selected_tag_values.set(selected);
                                                                    }
                                                                },
                                                            }
                                                            span { class: "text-xs", "{tag.label}:{tag.value}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    section { class: "space-y-2 border border-primary-6 bg-primary p-3",
                                        div { class: "text-xs font-mono text-secondary-5", "SINGLE ADD / EDIT" }
                                        button {
                                            class: "w-full rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90 disabled:opacity-50",
                                            disabled: create_or_edit_pending() || json_import_pending() || repo_editor_pending(),
                                            onclick: move |_| {
                                                let is_add_mode = matches!(panel_mode(), Some(ProjectPanelMode::Add));
                                                let repo_id = if let Some(ProjectPanelMode::Edit(project)) = panel_mode() {
                                                    project.repo_id
                                                } else {
                                                    form_repo_id().trim().to_string()
                                                };
                                                let name = form_name().trim().to_string();
                                                let slug = form_slug().trim().to_string();
                                                let description = form_description().to_string();
                                                if repo_id.is_empty() || name.is_empty() || slug.is_empty() {
                                                    *panel_message.write() = Some("必填字段不能为空（repo_id/name/slug）".to_string());
                                                    return;
                                                }
                                                *create_or_edit_pending.write() = true;
                                                *panel_message.write() = None;
                                                let item = ProjectImportItem {
                                                    id: None,
                                                    repo_id,
                                                    name,
                                                    slug,
                                                    description,
                                                    url: optional_text(form_url()),
                                                    avatar_url: optional_text(form_avatar_url()),
                                                    status: optional_text(form_status()),
                                                    logo: optional_text(form_logo()),
                                                    twitter: optional_text(form_twitter()),
                                                    tags: if is_add_mode {
                                                        let selected = dedup_values(form_selected_tag_values());
                                                        if selected.is_empty() {
                                                            None
                                                        } else {
                                                            Some(selected)
                                                        }
                                                    } else {
                                                        None
                                                    },
                                                };
                                                spawn(async move {
                                                    let result = if is_add_mode {
                                                        import_projects(vec![item]).await
                                                    } else {
                                                        update_projects(vec![item]).await
                                                    };
                                                    match result {
                                                        Ok(res) => {
                                                            *panel_message.write() = Some(format!(
                                                                "完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                                res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                                            ));
                                                            refresh.with_mut(|v| *v += 1);
                                                        }
                                                        Err(err) => *panel_message.write() = Some(err.to_string()),
                                                    }
                                                    *create_or_edit_pending.write() = false;
                                                });
                                            },
                                            if matches!(mode, ProjectPanelMode::Add) { "添加" } else { "保存更新" }
                                        }
                                        if create_or_edit_pending() {
                                            div { class: "text-xs text-secondary-5", "处理中..." }
                                        }
                                        if let Some(msg) = panel_message() {
                                            div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                        }
                                    }
                                    if matches!(mode, ProjectPanelMode::Add) {
                                        section { class: "space-y-2 border border-primary-6 bg-primary p-3",
                                            div { class: "text-xs font-mono text-secondary-5", "JSON IMPORT" }
                                            p { class: "text-xs text-secondary-5", "支持上传 JSON 文件并批量导入 project。" }
                                            input {
                                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs",
                                                r#type: "file",
                                                accept: ".json,application/json",
                                                disabled: json_import_pending() || create_or_edit_pending() || repo_editor_pending(),
                                                onchange: move |e| {
                                                    *json_import_message.write() = None;
                                                    let files = e.files();
                                                    let Some(file_data) = files.first().cloned() else {
                                                        *json_import_message.write() = Some("请选择 JSON 文件".to_string());
                                                        return;
                                                    };
                                                    let file_name = file_data.name();
                                                    json_file_name.set(file_name.clone());
                                                    spawn(async move {
                                                        match file_data.read_string().await {
                                                            Ok(text) => {
                                                                json_file_text.set(text);
                                                                *json_import_message.write() = Some(format!("已加载文件：{file_name}"));
                                                            }
                                                            Err(err) => {
                                                                json_file_text.set(String::new());
                                                                *json_import_message.write() = Some(format!("读取文件失败: {err}"));
                                                            }
                                                        }
                                                    });
                                                },
                                            }
                                            if !json_file_name().is_empty() {
                                                div { class: "text-xs text-secondary-5", "当前文件：{json_file_name}" }
                                            }
                                            button {
                                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                                                disabled: json_import_pending() || create_or_edit_pending() || repo_editor_pending(),
                                                onclick: move |_| {
                                                    let content = json_file_text();
                                                    if content.trim().is_empty() {
                                                        *json_import_message.write() = Some("请先选择并加载 JSON 文件".to_string());
                                                        return;
                                                    }
                                                    *json_import_pending.write() = true;
                                                    *json_import_message.write() = None;
                                                    spawn(async move {
                                                        match import_projects_json(content).await {
                                                            Ok(res) => {
                                                                *json_import_message.write() = Some(format!(
                                                                    "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                                    res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                                                ));
                                                                refresh.with_mut(|v| *v += 1);
                                                            }
                                                            Err(err) => *json_import_message.write() = Some(err.to_string()),
                                                        }
                                                        *json_import_pending.write() = false;
                                                    });
                                                },
                                                "导入 JSON"
                                            }
                                            if json_import_pending() {
                                                div { class: "text-xs text-secondary-5", "导入处理中..." }
                                            }
                                            if let Some(msg) = json_import_message() {
                                                div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                            }
                                        }
                                    }
                                }
                                TabContent {
                                    value: "repo".to_string(),
                                    index: 1usize,
                                    if let ProjectPanelMode::Edit(project) = mode.clone() {
                                        div { class: "space-y-3 border border-primary-6 bg-primary p-3",
                                            div { class: "text-xs font-mono text-secondary-5", "REPO TAGS EDITOR" }
                                            div { class: "text-xs text-secondary-5", "关联 Repo: {project.repo_id}" }
                                            button {
                                                class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                disabled: repo_editor_pending() || create_or_edit_pending() || json_import_pending(),
                                                onclick: {
                                                    let target_repo_id = project.repo_id.clone();
                                                    move |_| {
                                                        let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                                            *repo_editor_message.write() = Some(format!("无效 repo id: {target_repo_id}"));
                                                            repo_editor_current_tags.set(Vec::new());
                                                            repo_editor_selected_tags.set(Vec::new());
                                                            return;
                                                        };
                                                        *repo_editor_pending.write() = true;
                                                        *repo_editor_message.write() = None;
                                                        spawn(async move {
                                                            match get_repo(owner, name).await {
                                                                Ok(Some(repo_detail)) => {
                                                                    repo_editor_current_tags.set(repo_detail.tags);
                                                                    repo_editor_selected_tags.set(Vec::new());
                                                                    *repo_editor_message.write() = Some("已加载关联 repo tags".to_string());
                                                                }
                                                                Ok(None) => {
                                                                    repo_editor_current_tags.set(Vec::new());
                                                                    repo_editor_selected_tags.set(Vec::new());
                                                                    *repo_editor_message.write() = Some("未找到该 repo".to_string());
                                                                }
                                                                Err(err) => {
                                                                    repo_editor_current_tags.set(Vec::new());
                                                                    repo_editor_selected_tags.set(Vec::new());
                                                                    *repo_editor_message.write() = Some(err.to_string());
                                                                }
                                                            }
                                                            *repo_editor_pending.write() = false;
                                                        });
                                                    }
                                                },
                                                "加载当前 Repo Tags"
                                            }
                                            div { class: "text-xs font-semibold text-secondary-5", "当前已绑定 Tags" }
                                            if repo_editor_current_tags().is_empty() {
                                                div { class: "text-xs text-secondary-5", "（空）" }
                                            } else {
                                                div { class: "flex flex-wrap gap-2",
                                                    for tag in repo_editor_current_tags() {
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
                                                        input {
                                                            r#type: "checkbox",
                                                            checked: repo_editor_selected_tags().iter().any(|x| x.label == tag.label && x.value == tag.value),
                                                            onchange: {
                                                                let target_tag = tag.clone();
                                                                move |_| {
                                                                    let mut selected_tags = repo_editor_selected_tags();
                                                                    if let Some(index) = selected_tags.iter().position(|x| x == &target_tag) {
                                                                        selected_tags.remove(index);
                                                                    } else {
                                                                        selected_tags.push(target_tag.clone());
                                                                    }
                                                                    repo_editor_selected_tags.set(selected_tags);
                                                                }
                                                            },
                                                        }
                                                        span { class: "text-xs", "{tag.label}:{tag.value}" }
                                                    }
                                                }
                                            }
                                            div { class: "flex flex-wrap gap-2",
                                                button {
                                                    class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                    disabled: repo_editor_pending() || create_or_edit_pending() || json_import_pending(),
                                                    onclick: {
                                                        let target_repo_id = project.repo_id.clone();
                                                        move |_| {
                                                            let picked_tags = repo_editor_selected_tags();
                                                            if picked_tags.is_empty() {
                                                                *repo_editor_message.write() = Some("请先选择 tag".to_string());
                                                                return;
                                                            }
                                                            let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                                                *repo_editor_message.write() = Some(format!("无效 repo id: {target_repo_id}"));
                                                                return;
                                                            };
                                                            let mut next_tags = repo_editor_current_tags();
                                                            for tag in picked_tags {
                                                                if !next_tags.contains(&tag) {
                                                                    next_tags.push(tag);
                                                                }
                                                            }
                                                            *repo_editor_pending.write() = true;
                                                            *repo_editor_message.write() = None;
                                                            spawn(async move {
                                                                match replace_repo_tags(owner, name, next_tags.clone()).await {
                                                                    Ok(()) => {
                                                                        repo_editor_current_tags.set(next_tags);
                                                                        *repo_editor_message.write() = Some("Add 完成".to_string());
                                                                    }
                                                                    Err(err) => *repo_editor_message.write() = Some(err.to_string()),
                                                                }
                                                                *repo_editor_pending.write() = false;
                                                            });
                                                        }
                                                    },
                                                    "Add 到 Repo"
                                                }
                                                button {
                                                    class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                    disabled: repo_editor_pending() || create_or_edit_pending() || json_import_pending(),
                                                    onclick: {
                                                        let target_repo_id = project.repo_id.clone();
                                                        move |_| {
                                                            let picked_tags = repo_editor_selected_tags();
                                                            if picked_tags.is_empty() {
                                                                *repo_editor_message.write() = Some("请先选择 tag".to_string());
                                                                return;
                                                            }
                                                            let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
                                                                *repo_editor_message.write() = Some(format!("无效 repo id: {target_repo_id}"));
                                                                return;
                                                            };
                                                            let mut next_tags = repo_editor_current_tags();
                                                            next_tags.retain(|tag| !picked_tags.contains(tag));
                                                            *repo_editor_pending.write() = true;
                                                            *repo_editor_message.write() = None;
                                                            spawn(async move {
                                                                match replace_repo_tags(owner, name, next_tags.clone()).await {
                                                                    Ok(()) => {
                                                                        repo_editor_current_tags.set(next_tags);
                                                                        *repo_editor_message.write() = Some("Remove 完成".to_string());
                                                                    }
                                                                    Err(err) => *repo_editor_message.write() = Some(err.to_string()),
                                                                }
                                                                *repo_editor_pending.write() = false;
                                                            });
                                                        }
                                                    },
                                                    "从 Repo 移除"
                                                }
                                            }
                                            if repo_editor_pending() {
                                                div { class: "text-xs text-secondary-5", "处理中..." }
                                            }
                                            if let Some(msg) = repo_editor_message() {
                                                div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                            }
                                        }
                                    } else {
                                        div { class: "rounded-md border border-dashed border-primary-6 bg-primary px-3 py-6 text-center text-sm text-secondary-5",
                                            "请先创建 project 并进入 Edit 后再编辑关联 repo tags"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
