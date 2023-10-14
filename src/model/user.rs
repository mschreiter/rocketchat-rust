use serde::{Deserialize, Serialize};

use model::email::Email;
use model::user_status::UserStatus;
use model::user_type::UserType;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    user_name: String,
    name: String,
    roles: Vec<String>,
    email: Vec<Email>,
    created_at: String,
    last_login: String,
    #[serde(rename = "type")]
    user_type: UserType,
    user_status: UserStatus,
    active: bool,
    utc_offset: i32,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            user_name: String::new(),
            name: String::new(),
            roles: Vec::new(),
            email: Vec::new(),
            created_at: String::new(),
            last_login: String::new(),
            user_type: UserType::USER,
            user_status: UserStatus::OFFLINE,
            active: false,
            utc_offset: 0,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn roles(&self) -> &Vec<String> {
        &self.roles
    }
    pub fn email(&self) -> &Vec<Email> {
        &self.email
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn last_login(&self) -> &str {
        &self.last_login
    }
    pub fn user_type(&self) -> &UserType {
        &self.user_type
    }
    pub fn user_status(&self) -> &UserStatus {
        &self.user_status
    }
    pub fn active(&self) -> &bool {
        &self.active
    }
    pub fn utc_offset(&self) -> &i32 {
        &self.utc_offset
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_roles(&mut self, roles: Vec<String>) {
        self.roles = roles;
    }
    pub fn set_email(&mut self, email: Vec<Email>) {
        self.email = email;
    }
    pub fn set_created_at(&mut self, created_at: String) {
        self.created_at = created_at;
    }
    pub fn set_last_login(&mut self, last_login: String) {
        self.last_login = last_login;
    }
    pub fn set_user_type(&mut self, user_type: UserType) {
        self.user_type = user_type;
    }
    pub fn set_user_status(&mut self, user_status: UserStatus) {
        self.user_status = user_status;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn set_utc_offset(&mut self, utc_offset: i32) {
        self.utc_offset = utc_offset;
    }
}
