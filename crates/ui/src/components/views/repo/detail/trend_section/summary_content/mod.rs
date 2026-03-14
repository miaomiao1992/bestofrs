use dioxus::prelude::*;

use crate::components::icons::RustGearIcon;
use crate::IO::repos::list_repo_deltas_summary;
use crate::types::snapshot_deltas_summary::SnapshotMetricDeltaSummaryDto;

use super::super::RepoDetailContext;
use super::TrendContext;
pub(super) mod skeleton;

fn format_delta(value: i64) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

#[component]
pub(crate) fn TrendSummary() -> Element {
    let repo_ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let mut metric = trend_ctx.metric;
    let summary_fut = use_server_future(move || list_repo_deltas_summary((repo_ctx.owner)(), (repo_ctx.name)()))?;

    rsx! {
        div { class: "space-y-4",
            h2 { class: "text-5xl leading-[0.8] font-black tracking-tighter text-secondary-2 uppercase md:text-7xl",
                "Trend"
                br {}
                span { class: "text-transparent [-webkit-text-stroke:2px_var(--primary-color-6)]", "Analysis" }
            }
            div { class: "flex flex-wrap justify-center gap-2 mb-8",
                for item in ["stars", "forks", "issues"] {
                    button {
                        key: "{item}",
                        class: "px-4 py-2 text-xs font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                        class: if metric() == item {
                            "border border-secondary-2 bg-secondary-2 text-primary shadow-comic-sm"
                        } else {
                            "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                        },
                        onclick: move |_| metric.set(item.to_string()),
                        "{item}"
                    }
                }
            }
        }

        match summary_fut() {
            Some(Ok(summary)) => {
                let metric_summary: SnapshotMetricDeltaSummaryDto = match metric().as_str() {
                    "forks" => summary.forks,
                    "issues" => summary.issues,
                    _ => summary.stars,
                };
                let summary_rows = vec![
                    ("Daily".to_string(), metric_summary.daily),
                    ("Weekly".to_string(), metric_summary.weekly),
                    ("Monthly".to_string(), metric_summary.monthly),
                ];

                rsx! {
                    div { class: "grid grid-cols-1 gap-6 md:grid-cols-3 w-[64%] mx-auto",
                        for (label, value) in summary_rows {
                            div {
                                key: "{label}",
                                class: "relative flex min-h-[230px] items-center justify-center",
                                RustGearIcon {
                                    width: 180.0,
                                    class: "absolute text-primary-6",
                                }
                                div { class: "relative z-10 flex flex-col items-center gap-1 text-center",
                                    div { class: "mb-1 text-xs font-mono font-black tracking-[0.25em] text-secondary-5 uppercase", "{label}" }
                                    div {
                                        class: "text-3xl font-black",
                                        class: if value > 0 {
                                            "text-grid-accent"
                                        } else if value < 0 {
                                            "text-primary-error"
                                        } else {
                                            "text-secondary-3"
                                        },
                                        "{format_delta(value)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(Err(e)) => Err(e)?,
            None => rsx! { skeleton::TrendSummarySkeleton {} },
        }
    }
}
