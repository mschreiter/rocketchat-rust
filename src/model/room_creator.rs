use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomCreator {
    id: String,
    user_name: String,
}

impl RoomCreator {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            user_name: String::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }
}
