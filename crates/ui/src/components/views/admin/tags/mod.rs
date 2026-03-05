use dioxus::prelude::*;

use crate::components::common::CommonPagination;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::IO::repos::{
    bulk_update_repo_tag, create_tag, delete_tag, import_tags_json, list_tags_with_meta,
    search_repos, update_tag,
};
use crate::types::search::SearchResultDto;
use crate::types::tags::TagListItemDto;
use app::prelude::Pagination;

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

fn empty_search_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}


#[derive(Clone, PartialEq)]
enum TagPanelMode {
    Add,
    Edit(TagListItemDto),
}

#[component]
pub fn Tags() -> Element {
    let mut refresh = use_signal(|| 0u32);
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let mut search_key = use_signal(String::new);
    let mut table_page = use_signal(|| 1u32);
    let page_size = 20usize;
    let mut panel_mode = use_signal(|| Option::<TagPanelMode>::None);
    let mut panel_tab = use_signal(|| Some("base".to_string()));
    let panel_tab_read: ReadSignal<Option<String>> = panel_tab.into();
    let mut action_pending = use_signal(|| false);
    let mut panel_message = use_signal(|| Option::<String>::None);
    let mut table_message = use_signal(|| Option::<String>::None);

    let mut form_label = use_signal(String::new);
    let mut form_value = use_signal(String::new);
    let mut form_description = use_signal(String::new);
    let mut json_import_pending = use_signal(|| false);
    let mut json_import_message = use_signal(|| Option::<String>::None);
    let mut json_file_name = use_signal(String::new);
    let mut json_file_text = use_signal(String::new);

    let group_page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let mut repo_search_key = use_signal(String::new);
    let mut selected_repo_ids = use_signal(Vec::<String>::new);
    let mut bulk_message = use_signal(|| Option::<String>::None);
    let mut bulk_pending = use_signal(|| false);
    let mut repo_search = use_action({
        let page = group_page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_search_result(page));
            }
            search_repos(key, page).await
        }
    });

    let panel_open = panel_mode().is_some();
    let key = search_key().trim().to_lowercase();

    let table_items = match tags_page() {
        Some(Ok(page)) => page
            .items
            .into_iter()
            .filter(|tag| {
                if key.is_empty() {
                    return true;
                }
                let joined = format!(
                    "{}:{} {}",
                    tag.label.to_lowercase(),
                    tag.value.to_lowercase(),
                    tag.description.clone().unwrap_or_default().to_lowercase()
                );
                joined.contains(&key)
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    let (paged_items, total_pages) = paginate_items(&table_items, table_page(), page_size);
    let total_items = table_items.len() as u32;

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "TAGS / MANAGEMENT" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Tag 管理" }
                p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                    "右侧面板打开时，左侧只保留 tag 与 edit，便于快速切换。"
                }
            }
            div { class: "flex flex-col gap-2 md:flex-row",
                input {
                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                    placeholder: "搜索 label / value / description",
                    value: search_key,
                    oninput: move |e| {
                        *search_key.write() = e.value();
                        table_page.set(1);
                    },
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
                        panel_mode.set(Some(TagPanelMode::Add));
                        panel_tab.set(Some("base".to_string()));
                        panel_message.set(None);
                        json_import_message.set(None);
                        json_file_name.set(String::new());
                        json_file_text.set(String::new());
                        form_label.set(String::new());
                        form_value.set(String::new());
                        form_description.set(String::new());
                    },
                    "Add Tag"
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
                                    th { class: "px-3 py-2 text-left font-medium text-secondary-5", "tag" }
                                    if !panel_open {
                                        th { class: "px-3 py-2 text-left font-medium text-secondary-5", "description" }
                                    }
                                    th { class: "px-3 py-2 text-right font-medium text-secondary-5", "actions" }
                                }
                            }
                            tbody {
                                match tags_page() {
                                    Some(Err(err)) => rsx! {
                                        tr { td { class: "px-3 py-6 text-center text-primary-error", colspan: if panel_open { "2" } else { "3" }, "{err}" } }
                                    },
                                    None => rsx! {
                                        tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "3" }, "Loading..." } }
                                    },
                                    Some(Ok(_)) => {
                                        if paged_items.is_empty() {
                                            rsx! {
                                                tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if panel_open { "2" } else { "3" }, "无匹配结果" } }
                                            }
                                        } else {
                                            rsx! {
                                                for tag in paged_items {
                                                    tr { key: "{tag.label}:{tag.value}", class: "border-b border-primary-6 last:border-b-0",
                                                        td { class: "px-3 py-2 font-mono text-xs", "{tag.label}:{tag.value}" }
                                                        if !panel_open {
                                                            td { class: "px-3 py-2 text-secondary-5 max-w-[320px] truncate", "{tag.description.clone().unwrap_or_default()}" }
                                                        }
                                                        td { class: "px-3 py-2",
                                                            div { class: "flex justify-end gap-2",
                                                                button {
                                                                    class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                                    disabled: action_pending(),
                                                                    onclick: {
                                                                        let t = tag.clone();
                                                                        move |_| {
                                                                            panel_mode.set(Some(TagPanelMode::Edit(t.clone())));
                                                                            panel_tab.set(Some("base".to_string()));
                                                                            panel_message.set(None);
                                                                            form_label.set(t.label.clone());
                                                                            form_value.set(t.value.clone());
                                                                            form_description.set(t.description.clone().unwrap_or_default());
                                                                            selected_repo_ids.set(Vec::new());
                                                                            bulk_message.set(None);
                                                                        }
                                                                    },
                                                                    "Edit"
                                                                }
                                                                if !panel_open {
                                                                    button {
                                                                        class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                                        disabled: action_pending(),
                                                                        onclick: {
                                                                            let t = tag.clone();
                                                                            move |_| {
                                                                                let label = t.label.clone();
                                                                                let value = t.value.clone();
                                                                                *action_pending.write() = true;
                                                                                *table_message.write() = None;
                                                                                spawn(async move {
                                                                                    match delete_tag(label.clone(), value.clone()).await {
                                                                                        Ok(()) => {
                                                                                            *table_message.write() = Some(format!("已删除 tag: {}:{}", label, value));
                                                                                            refresh.with_mut(|v| *v += 1);
                                                                                        }
                                                                                        Err(err) => *table_message.write() = Some(err.to_string()),
                                                                                    }
                                                                                    *action_pending.write() = false;
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
                                }
                            }
                        }
                    }

                    div { class: "text-xs text-secondary-5", "{total_items} items" }
                    if action_pending() {
                        div { class: "text-xs text-secondary-5", "处理中..." }
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
                                    if matches!(mode, TagPanelMode::Add) { "Add Tag" } else { "Edit Tag" }
                                }
                                button {
                                    class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                    disabled: action_pending() || bulk_pending() || json_import_pending(),
                                    onclick: move |_| {
                                        panel_mode.set(None);
                                        panel_message.set(None);
                                        bulk_message.set(None);
                                        json_import_message.set(None);
                                    },
                                    "关闭"
                                }
                            }

                            Tabs {
                                class: "space-y-3".to_string(),
                                value: panel_tab_read,
                                default_value: "base".to_string(),
                                on_value_change: move |value| panel_tab.set(Some(value)),
                                TabList {
                                    TabTrigger { value: "base".to_string(), index: 0usize, "base" }
                                    TabTrigger { value: "group".to_string(), index: 1usize, "group" }
                                }
                                TabContent {
                                    value: "base".to_string(),
                                    index: 0usize,
                                    if let TagPanelMode::Edit(ref tag) = mode {
                                        div { class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs text-secondary-5",
                                            "editing: {tag.label}:{tag.value}"
                                        }
                                    }

                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "label *",
                                        value: form_label,
                                        disabled: matches!(mode, TagPanelMode::Edit(_)),
                                        oninput: move |e| *form_label.write() = e.value(),
                                    }
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "value *",
                                        value: form_value,
                                        disabled: matches!(mode, TagPanelMode::Edit(_)),
                                        oninput: move |e| *form_value.write() = e.value(),
                                    }
                                    textarea {
                                        class: "w-full min-h-[120px] rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "description",
                                        value: form_description,
                                        oninput: move |e| *form_description.write() = e.value(),
                                    }

                                    {
                                        let is_add_mode = matches!(mode, TagPanelMode::Add);
                                        rsx! {
                                            button {
                                                class: "w-full rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90 disabled:opacity-50",
                                                disabled: action_pending() || json_import_pending(),
                                                onclick: move |_| {
                                                    let label = form_label().trim().to_string();
                                                    let value = form_value().trim().to_string();
                                                    if label.is_empty() || value.is_empty() {
                                                        *panel_message.write() = Some("label/value 不能为空".to_string());
                                                        return;
                                                    }
                                                    let description = form_description().trim().to_string();
                                                    let description = if description.is_empty() { None } else { Some(description) };
                                                    *action_pending.write() = true;
                                                    *panel_message.write() = None;
                                                    if is_add_mode {
                                                        spawn(async move {
                                                            match create_tag(label.clone(), value.clone()).await {
                                                                Ok(()) => match update_tag(label.clone(), value.clone(), description).await {
                                                                    Ok(()) => {
                                                                        *panel_message.write() = Some("创建成功".to_string());
                                                                        refresh.with_mut(|v| *v += 1);
                                                                    }
                                                                    Err(err) => *panel_message.write() = Some(err.to_string()),
                                                                },
                                                                Err(err) => *panel_message.write() = Some(err.to_string()),
                                                            }
                                                            *action_pending.write() = false;
                                                        });
                                                    } else {
                                                        spawn(async move {
                                                            match update_tag(label.clone(), value.clone(), description).await {
                                                                Ok(()) => {
                                                                    *panel_message.write() = Some("更新成功".to_string());
                                                                    refresh.with_mut(|v| *v += 1);
                                                                }
                                                                Err(err) => *panel_message.write() = Some(err.to_string()),
                                                            }
                                                            *action_pending.write() = false;
                                                        });
                                                    }
                                                },
                                                if is_add_mode { "添加" } else { "保存更新" }
                                            }
                                        }
                                    }

                                    if let Some(msg) = panel_message() {
                                        div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                    }
                                    if matches!(mode, TagPanelMode::Add) {
                                        section { class: "space-y-2 border border-primary-6 bg-primary p-3",
                                            div { class: "text-xs font-mono text-secondary-5", "JSON IMPORT" }
                                            p { class: "text-xs text-secondary-5", "支持上传 JSON 文件并批量导入 tags。" }
                                            input {
                                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs",
                                                r#type: "file",
                                                accept: ".json,application/json",
                                                disabled: json_import_pending() || action_pending(),
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
                                                disabled: json_import_pending() || action_pending(),
                                                onclick: move |_| {
                                                    let content = json_file_text();
                                                    if content.trim().is_empty() {
                                                        *json_import_message.write() = Some("请先选择并加载 JSON 文件".to_string());
                                                        return;
                                                    }
                                                    *json_import_pending.write() = true;
                                                    *json_import_message.write() = None;
                                                    spawn(async move {
                                                        match import_tags_json(content).await {
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
                                    value: "group".to_string(),
                                    index: 1usize,
                                    if matches!(mode, TagPanelMode::Add) {
                                        div { class: "rounded-md border border-dashed border-primary-6 bg-primary px-3 py-6 text-center text-sm text-secondary-5",
                                            "请先从左侧列表选择一个 tag 进入 Edit 后再使用 group 功能"
                                        }
                                    } else if let TagPanelMode::Edit(current_tag) = mode.clone() {
                                        div { class: "space-y-4",

                                            section { class: "space-y-3 border border-primary-6 bg-primary p-3",
                                                div { class: "text-xs font-mono text-secondary-5", "TAG BULK TO REPOS / {current_tag.label}:{current_tag.value}" }
                                                div { class: "flex flex-col gap-2 md:flex-row",
                                                    input {
                                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                                        placeholder: "搜索 repo（owner/name）",
                                                        value: repo_search_key,
                                                        oninput: move |e| *repo_search_key.write() = e.value(),
                                                        onkeydown: move |e| {
                                                            if e.key() == Key::Enter {
                                                                repo_search.call(repo_search_key());
                                                            }
                                                        },
                                                    }
                                                    button {
                                                        class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                                                        onclick: move |_| repo_search.call(repo_search_key()),
                                                        "搜索 Repo"
                                                    }
                                                }

                                                if let Some(Ok(result)) = repo_search.value() {
                                                    {
                                                        let repos_for_select_all = result().repos.items.clone();
                                                        let repos_for_list = result().repos.items.clone();
                                                        rsx! {
                                                            div { class: "space-y-2",
                                                                div { class: "flex items-center gap-2",
                                                                    button {
                                                                        class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                                                        onclick: move |_| {
                                                                            let mut ids = selected_repo_ids();
                                                                            for repo in &repos_for_select_all {
                                                                                if !ids.contains(&repo.id) {
                                                                                    ids.push(repo.id.clone());
                                                                                }
                                                                            }
                                                                            selected_repo_ids.set(ids);
                                                                        },
                                                                        "全选当前结果"
                                                                    }
                                                                    button {
                                                                        class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                                                        onclick: move |_| selected_repo_ids.set(Vec::new()),
                                                                        "清空"
                                                                    }
                                                                    span { class: "text-xs text-secondary-5", "已选 {selected_repo_ids().len()} 项" }
                                                                }
                                                                div { class: "max-h-[180px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                                                                    for repo in repos_for_list {
                                                                        label { key: "bulk-{repo.id}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                                                            input {
                                                                                r#type: "checkbox",
                                                                                checked: selected_repo_ids().contains(&repo.id),
                                                                                onchange: move |_| {
                                                                                    let mut ids = selected_repo_ids();
                                                                                    if let Some(idx) = ids.iter().position(|id| *id == repo.id) {
                                                                                        ids.remove(idx);
                                                                                    } else {
                                                                                        ids.push(repo.id.clone());
                                                                                    }
                                                                                    selected_repo_ids.set(ids);
                                                                                },
                                                                            }
                                                                            span { class: "text-xs", "{repo.id}" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                {
                                                    let label_for_add = current_tag.label.clone();
                                                    let value_for_add = current_tag.value.clone();
                                                    let label_for_remove = current_tag.label.clone();
                                                    let value_for_remove = current_tag.value.clone();
                                                    rsx! {
                                                        div { class: "flex flex-wrap gap-2",
                                                            button {
                                                                class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                                disabled: bulk_pending(),
                                                                onclick: move |_| {
                                                                    let repo_ids = selected_repo_ids();
                                                                    if repo_ids.is_empty() {
                                                                        *bulk_message.write() = Some("请先选择 repo".to_string());
                                                                        return;
                                                                    }
                                                                    *bulk_pending.write() = true;
                                                                    *bulk_message.write() = None;
                                                                    let target_label = label_for_add.clone();
                                                                    let target_value = value_for_add.clone();
                                                                    spawn(async move {
                                                                        match bulk_update_repo_tag(repo_ids, target_label, target_value, "add".to_string()).await {
                                                                            Ok(res) => *bulk_message.write() = Some(format!(
                                                                                "Add 完成: total={} updated={} skipped={}",
                                                                                res.total, res.updated, res.skipped
                                                                            )),
                                                                            Err(err) => *bulk_message.write() = Some(err.to_string()),
                                                                        }
                                                                        *bulk_pending.write() = false;
                                                                    });
                                                                },
                                                                "批量 Add"
                                                            }
                                                            button {
                                                                class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                                disabled: bulk_pending(),
                                                                onclick: move |_| {
                                                                    let repo_ids = selected_repo_ids();
                                                                    if repo_ids.is_empty() {
                                                                        *bulk_message.write() = Some("请先选择 repo".to_string());
                                                                        return;
                                                                    }
                                                                    *bulk_pending.write() = true;
                                                                    *bulk_message.write() = None;
                                                                    let target_label = label_for_remove.clone();
                                                                    let target_value = value_for_remove.clone();
                                                                    spawn(async move {
                                                                        match bulk_update_repo_tag(repo_ids, target_label, target_value, "remove".to_string()).await {
                                                                            Ok(res) => *bulk_message.write() = Some(format!(
                                                                                "Remove 完成: total={} updated={} skipped={}",
                                                                                res.total, res.updated, res.skipped
                                                                            )),
                                                                            Err(err) => *bulk_message.write() = Some(err.to_string()),
                                                                        }
                                                                        *bulk_pending.write() = false;
                                                                    });
                                                                },
                                                                "批量 Remove"
                                                            }
                                                        }
                                                    }
                                                }
                                                if bulk_pending() {
                                                    div { class: "text-xs text-secondary-5", "处理中..." }
                                                }
                                                if let Some(msg) = bulk_message() {
                                                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
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
    }
}
