use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DurationRange {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl DurationRange {
    pub fn days(self) -> i64 {
        match self {
            DurationRange::Daily => 1,
            DurationRange::Weekly => 7,
            DurationRange::Monthly => 30,
            DurationRange::Yearly => 365,
        }
    }
}
