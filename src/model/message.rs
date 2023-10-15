use serde::{Deserialize, Serialize};

use crate::model::attachment::Attachment;
use crate::model::emoji::Emoji;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    id: String,
    attachments: Vec<Attachment>,
    emoji: Emoji,
    room_id: String,
    text: String,
    alias: String,
    avatar: String,
}

impl Message {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            attachments: Vec::new(),
            emoji: Emoji::new(),
            room_id: String::new(),
            text: String::new(),
            alias: String::new(),
            avatar: String::new(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn attachments(&self) -> &Vec<Attachment> {
        &self.attachments
    }
    pub fn emoji(&self) -> &Emoji {
        &self.emoji
    }
    pub fn room_id(&self) -> &str {
        &self.room_id
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn alias(&self) -> &str {
        &self.alias
    }
    pub fn avatar(&self) -> &str {
        &self.avatar
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}
