use dioxus::prelude::*;

use crate::types::repos::RepoDto;

#[component]
pub fn RepoMetaSection(repo_fut: Resource<Result<Option<RepoDto>, ServerFnError>>, owner: String, name: String) -> Element {
    rsx! {
        section { class: "rounded-xl border border-primary-6 bg-primary-2 p-5 space-y-4",
            h2 { class: "text-lg font-semibold", "Meta" }
            match repo_fut() {
                Some(Ok(Some(repo))) => {
                    let github_url = format!("https://github.com/{owner}/{name}");
                    rsx! {
                        div { class: "grid grid-cols-1 gap-2 text-sm sm:grid-cols-2",
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "id" }
                                span { class: "font-medium", "{repo.id}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "github_repo_id" }
                                span { class: "font-medium", "{repo.github_repo_id:?}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "stars" }
                                span { class: "font-medium", "{repo.stars}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "forks" }
                                span { class: "font-medium", "{repo.forks}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "open_issues" }
                                span { class: "font-medium", "{repo.open_issues}" }
                            }
                            div { class: "flex items-center justify-between",
                                span { class: "text-secondary-5", "watchers" }
                                span { class: "font-medium", "{repo.watchers}" }
                            }
                        }

                        div { class: "text-sm text-secondary-5",
                            "source: "
                            a { class: "hover:underline", href: "{github_url}", target: "_blank", "{github_url}" }
                        }

                        if let Some(full_name) = repo.full_name {
                            div { class: "text-sm text-secondary-5", "full_name: {full_name}" }
                        }
                        if let Some(last) = repo.last_fetched_at {
                            div { class: "text-sm text-secondary-5", "last_fetched_at: {last}" }
                        }
                        if !repo.tags.is_empty() {
                            div { class: "flex flex-wrap gap-2 text-xs",
                                for tag in repo.tags {
                                    span { class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-0.5",
                                        "{tag.label}:{tag.value}"
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Ok(None)) => rsx! { div { class: "text-sm text-secondary-5", "未找到 repo" } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
            }
        }
    }
}
