use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    email: String,
    name: String,
    password: String,
    username: String,
    verified: bool,
    roles: Vec<String>,
}

impl CreateUserRequest {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            name: String::new(),
            password: String::new(),
            username: String::new(),
            verified: false,
            roles: Vec::new(),
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
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn verified(&self) -> bool {
        self.verified
    }
    pub fn roles(&self) -> &Vec<String> {
        &self.roles
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
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_verified(&mut self, verified: bool) {
        self.verified = verified;
    }
    pub fn set_roles(&mut self, roles: Vec<String>) {
        self.roles = roles;
    }
}
