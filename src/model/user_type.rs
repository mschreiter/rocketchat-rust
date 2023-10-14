use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum UserType {
    #[default]
    #[serde(rename = "user")]
    USER,
    #[serde(rename = "bot")]
    BOT,
}
