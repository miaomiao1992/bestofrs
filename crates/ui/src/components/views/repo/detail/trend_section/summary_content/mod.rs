use dioxus::prelude::*;

use crate::components::icons::RustGearIcon;
use crate::IO::repos::list_repo_deltas_summary;
use crate::types::snapshot_deltas_summary::SnapshotMetricDeltaSummaryDto;

use super::super::RepoDetailContext;
pub(super) mod skeleton;

fn format_delta(value: i64) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

#[component]
pub(crate) fn TrendSummary(metric: Signal<String>) -> Element {
    let repo_ctx = use_context::<RepoDetailContext>();
    let summary_fut = use_server_future(move || list_repo_deltas_summary((repo_ctx.owner)(), (repo_ctx.name)()))?;

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
