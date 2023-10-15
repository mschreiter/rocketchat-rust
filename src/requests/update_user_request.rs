use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    email: String,
    name: String,
    password: String,
    active: bool,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    user_id: String,
    data: Data,
}

impl UpdateUserRequest {
    pub fn new() -> Self {
        Self {
            user_id: String::new(),
            data: Data::new(),
        }
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn data(&self) -> &Data {
        &self.data
    }
    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }
    pub fn set_data(&mut self, data: Data) {
        self.data = data;
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            name: String::new(),
            password: String::new(),
            active: false,
        }
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
