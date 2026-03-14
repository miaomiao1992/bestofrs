use app::snapshot::{SnapshotDeltasSummary, SnapshotMetricDeltaSummary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotMetricDeltaSummaryDto {
    pub daily: i64,
    pub weekly: i64,
    pub monthly: i64,
}

impl From<SnapshotMetricDeltaSummary> for SnapshotMetricDeltaSummaryDto {
    fn from(value: SnapshotMetricDeltaSummary) -> Self {
        Self {
            daily: value.daily,
            weekly: value.weekly,
            monthly: value.monthly,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotDeltasSummaryDto {
    pub stars: SnapshotMetricDeltaSummaryDto,
    pub forks: SnapshotMetricDeltaSummaryDto,
    pub issues: SnapshotMetricDeltaSummaryDto,
}

impl From<SnapshotDeltasSummary> for SnapshotDeltasSummaryDto {
    fn from(value: SnapshotDeltasSummary) -> Self {
        Self {
            stars: value.stars.into(),
            forks: value.forks.into(),
            issues: value.issues.into(),
        }
    }
}
