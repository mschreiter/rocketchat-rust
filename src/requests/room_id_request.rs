use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomIdRequest {
    room_id: String,
}

impl RoomIdRequest {
    pub fn new() -> Self {
        Self {
            room_id: String::new(),
        }
    }
    pub fn room_id(&self) -> &str {
        &self.room_id
    }
    pub fn set_room_id(&mut self, room_id: String) {
        self.room_id = room_id;
    }
}
