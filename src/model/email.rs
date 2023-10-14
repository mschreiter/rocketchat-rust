use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Email {
    address: String,
    verified: bool,
}

impl Email {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            verified: false,
        }
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn set_verified(&mut self, verified: bool) {
        self.verified = verified;
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn verified(&self) -> bool {
        self.verified
    }
}
