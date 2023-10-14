use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum RoomType {
    #[default]
    #[serde(rename = "c")]
    PublicChannel,
    #[serde(rename = "d")]
    DirectMessage,
    #[serde(rename = "p")]
    PrivateGroup,
}
