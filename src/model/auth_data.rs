use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthData {
    auth_token: String,
    user_id: String,
}

impl AuthData {
    pub fn new() -> Self {
        Self {
            auth_token: String::new(),
            user_id: String::new(),
        }
    }

    pub fn auth_token(&self) -> &str {
        &self.auth_token
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn set_auth_token(&mut self, auth_token: String) {
        self.auth_token = auth_token;
    }
    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }
}
