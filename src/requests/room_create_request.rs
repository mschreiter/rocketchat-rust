use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomCreateRequest {
    name: String,
    members: Vec<String>,
}

impl RoomCreateRequest {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            members: Vec::new(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn members(&self) -> &Vec<String> {
        &self.members
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_members(&mut self, members: Vec<String>) {
        self.members = members;
    }
}
