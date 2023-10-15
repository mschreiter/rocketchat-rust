use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    id: String,
    value: String,
}

impl Setting {
    pub fn new() -> Self {
        Setting {
            id: String::new(),
            value: String::new(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}
