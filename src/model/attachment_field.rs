use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentField {
    is_short: bool,
    title: String,
    value: String,
}
impl AttachmentField {
    pub fn new() -> Self {
        Self {
            is_short: false,
            title: String::new(),
            value: String::new(),
        }
    }

    pub fn set_is_short(&mut self, is_short: bool) {
        self.is_short = is_short;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn is_short(&self) -> bool {
        self.is_short
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
