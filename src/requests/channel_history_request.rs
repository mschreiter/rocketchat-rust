use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHistoryRequest {
    room_id: String,
    latest: String,
    oldest: String,
    inclusive: bool,
}

impl ChannelHistoryRequest {
    pub fn new() -> Self {
        Self {
            room_id: String::new(),
            latest: String::new(),
            oldest: String::new(),
            inclusive: false,
        }
    }
    pub fn room_id(&self) -> &str {
        &self.room_id
    }
    pub fn latest(&self) -> &str {
        &self.latest
    }
    pub fn oldest(&self) -> &str {
        &self.oldest
    }
    pub fn inclusive(&self) -> bool {
        self.inclusive
    }
    pub fn set_room_id(&mut self, room_id: String) {
        self.room_id = room_id;
    }
    pub fn set_latest(&mut self, latest: String) {
        self.latest = latest;
    }
    pub fn set_oldest(&mut self, oldest: String) {
        self.oldest = oldest;
    }
    pub fn set_inclusive(&mut self, inclusive: bool) {
        self.inclusive = inclusive;
    }
}
