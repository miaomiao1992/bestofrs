pub(super) mod delta_content;
pub(super) mod snapshot_content;
pub(super) mod summary_content;

use crate::components::common::IOCell;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant};
use dioxus::prelude::*;
use serde_json::Value;

use delta_content::skeleton::DeltaContentSkeleton;
use delta_content::DeltaContent;
use snapshot_content::skeleton::SnapshotContentSkeleton;
use snapshot_content::SnapshotContent;
use summary_content::skeleton::TrendSummarySkeleton;
use summary_content::TrendSummary;

#[derive(Clone, Copy)]
pub(super) struct TrendContext {
    pub(super) metric: Signal<String>,
    pub(super) delta_timeframe: Signal<String>,
    pub(super) snapshot_timeframe: Signal<String>,
}

fn metric_dataset_label(metric: &str, chart_kind: &str) -> &'static str {
    match chart_kind {
        "delta" => match metric {
            "forks" => "forks_delta",
            "issues" => "open_issues_delta",
            _ => "stars_delta",
        },
        _ => match metric {
            "forks" => "forks",
            "issues" => "open_issues",
            "watchers" => "watchers",
            _ => "stars",
        },
    }
}

pub(super) fn apply_metric_visibility(mut config: Value, metric: &str, chart_kind: &str) -> Value {
    let active_label = metric_dataset_label(metric, chart_kind);
    if let Some(datasets) = config
        .get_mut("data")
        .and_then(|data| data.get_mut("datasets"))
        .and_then(|datasets| datasets.as_array_mut())
    {
        for dataset in datasets {
            let is_active = dataset
                .get("label")
                .and_then(|label| label.as_str())
                .map(|label| label == active_label)
                .unwrap_or(false);
            if let Some(dataset_obj) = dataset.as_object_mut() {
                dataset_obj.insert("hidden".to_string(), Value::Bool(!is_active));
            }
        }
    }
    config
}


#[component]
pub(crate) fn TrendSection() -> Element {
    let metric = use_signal(|| "stars".to_string());
    let mut delta_timeframe = use_signal(|| "weekly".to_string());
    let mut snapshot_timeframe = use_signal(|| "monthly".to_string());
    use_context_provider(|| TrendContext {
        metric,
        delta_timeframe,
        snapshot_timeframe,
    });
    let mut active_tab = use_signal(|| Some("delta".to_string()));
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();

    rsx! {
        section { class: "space-y-6",
            IOCell {
                loading_fallback: rsx! { TrendSummarySkeleton {} },
                TrendSummary {}
            }

            Tabs {
                class: "space-y-4".to_string(),
                variant: TabsVariant::Ghost,
                value: active_tab_read,
                default_value: "delta".to_string(),
                on_value_change: move |value| active_tab.set(Some(value)),
                TabList {
                    TabTrigger { value: "delta".to_string(), index: 0usize, "DELTAS" }
                    TabTrigger { value: "snapshot".to_string(), index: 1usize, "SNAPSHOT" }
                }
                TabContent {
                    value: "delta".to_string(),
                    index: 0usize,
                    class: "flex h-[24rem] flex-col gap-4 border border-primary-6 bg-primary-1 p-4 shadow-comic-sm".to_string(),
                    div { class: "flex justify-end",
                        div { class: "flex gap-2",
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if delta_timeframe() == "weekly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| delta_timeframe.set("weekly".to_string()),
                                "Weekly"
                            }
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if delta_timeframe() == "monthly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| delta_timeframe.set("monthly".to_string()),
                                "Monthly"
                            }
                        }
                    }
                    div { class: "min-h-0 flex-1",
                        IOCell {
                            loading_fallback: rsx! { DeltaContentSkeleton {} },
                            DeltaContent {}
                        }
                    }
                }
                TabContent {
                    value: "snapshot".to_string(),
                    index: 1usize,
                    class: "flex h-[24rem] flex-col gap-4 border border-primary-6 bg-primary-1 p-4 shadow-comic-sm".to_string(),
                    div { class: "flex justify-end",
                        div { class: "flex gap-2",
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if snapshot_timeframe() == "monthly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| snapshot_timeframe.set("monthly".to_string()),
                                "Monthly"
                            }
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if snapshot_timeframe() == "yearly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| snapshot_timeframe.set("yearly".to_string()),
                                "Yearly"
                            }
                        }
                    }
                    div { class: "min-h-0 flex-1",
                        IOCell {
                            loading_fallback: rsx! { SnapshotContentSkeleton {} },
                            SnapshotContent {}
                        }
                    }
                }
            }
        }
    }
}
