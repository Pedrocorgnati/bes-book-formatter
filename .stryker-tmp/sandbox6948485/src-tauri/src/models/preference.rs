use serde::{Deserialize, Serialize};

/// User preference key-value pair from `user_preferences` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preference {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}
