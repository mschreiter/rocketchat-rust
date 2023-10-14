use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "online")]
    ONLINE,
    #[serde(rename = "busy")]
    BUSY,
    #[serde(rename = "away")]
    AWAY,
    #[default]
    #[serde(rename = "offline")]
    OFFLINE,
}
