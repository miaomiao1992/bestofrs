use dioxus::prelude::*;

use crate::IO::admin::run_ingest_daily_snapshots;
use app::prelude::IngestDailySnapshotsResult;

pub(super) mod skeleton;

#[component]
pub(super) fn Result(run_nonce: ReadSignal<u32>) -> Element {
    let run_result = use_server_future(move || {
        let n = run_nonce();
        async move {
            if n == 0 {
                Ok::<Option<IngestDailySnapshotsResult>, ServerFnError>(None)
            } else {
                run_ingest_daily_snapshots().await.map(Some)
            }
        }
    })?;

    match run_result() {
        Some(Ok(Some(res))) => rsx! {
            div { class: "grid grid-cols-1 gap-2 text-sm",
                div { class: "flex items-center justify-between",
                    span { class: "text-secondary-5", "projects" }
                    span { class: "font-medium", "{res.projects}" }
                }
                div { class: "flex items-center justify-between",
                    span { class: "text-secondary-5", "repos_upserted" }
                    span { class: "font-medium", "{res.repos_upserted}" }
                }
                div { class: "flex items-center justify-between",
                    span { class: "text-secondary-5", "snapshots_inserted" }
                    span { class: "font-medium", "{res.snapshots_inserted}" }
                }
            }
        },
        Some(Ok(None)) => rsx! {
            div { class: "text-sm text-secondary-5", "尚未运行" }
        },
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::ResultSkeleton {} },
    }
}
