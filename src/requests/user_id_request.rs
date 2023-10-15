use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdRequest {
    user_name: String,
    user_id: String,
}

impl UserIdRequest {
    pub fn new() -> Self {
        Self {
            user_name: String::new(),
            user_id: String::new(),
        }
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }
    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }
}
