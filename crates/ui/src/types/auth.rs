use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeDto {
    pub user_id: String,
    pub login: String,
    pub avatar_url: Option<String>,
    pub role: String,
}
