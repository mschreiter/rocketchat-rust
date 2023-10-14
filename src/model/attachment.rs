use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    color: String,
    text: String,
    ts: String,
    thumb_url: String,
    message_link: String,
}

impl Attachment {
    pub fn new() -> Self {
        Self {
            color: String::new(),
            text: String::new(),
            ts: String::new(),
            thumb_url: String::new(),
            message_link: String::new(),
        }
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_ts(&mut self, ts: String) {
        self.ts = ts;
    }

    pub fn set_thumb_url(&mut self, thumb_url: String) {
        self.thumb_url = thumb_url;
    }

    pub fn set_message_link(&mut self, message_link: String) {
        self.message_link = message_link;
    }

    pub fn color(&self) -> &String {
        &self.color
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn ts(&self) -> &String {
        &self.ts
    }

    pub fn thumb_url(&self) -> &String {
        &self.thumb_url
    }

    pub fn message_link(&self) -> &String {
        &self.message_link
    }
}
