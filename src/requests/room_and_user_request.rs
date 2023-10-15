use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomAndUserRequest {
    pub room_id: String,
    pub user_id: String,
}

impl RoomAndUserRequest {
    pub fn new() -> Self {
        Self {
            room_id: String::new(),
            user_id: String::new(),
        }
    }
    pub fn room_id(&self) -> &str {
        &self.room_id
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn set_room_id(&mut self, room_id: String) {
        self.room_id = room_id;
    }
    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }
}
