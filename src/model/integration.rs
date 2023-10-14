use serde::{Deserialize, Serialize};

use model::integration_type::IntegrationType;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    #[serde(rename = "type")]
    integration_type: IntegrationType,
    name: String,
    username: String,
    channel: String,
    trigger_words: String,
    alias: String,
    avatar: String,
    emoji: String,
    token: String,
    script: String,
    urls: Vec<String>,
    enabled: bool,
    script_enabled: bool,
}

impl Integration {
    pub fn new() -> Self {
        Self {
            integration_type: IntegrationType::IncomingWebhook,
            name: String::new(),
            username: String::new(),
            channel: String::new(),
            trigger_words: String::new(),
            alias: String::new(),
            avatar: String::new(),
            emoji: String::new(),
            token: String::new(),
            script: String::new(),
            urls: Vec::new(),
            enabled: false,
            script_enabled: false,
        }
    }
    pub fn integration_type(&self) -> &IntegrationType {
        &self.integration_type
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn channel(&self) -> &str {
        &self.channel
    }
    pub fn trigger_words(&self) -> &str {
        &self.trigger_words
    }
    pub fn alias(&self) -> &str {
        &self.alias
    }
    pub fn avatar(&self) -> &str {
        &self.avatar
    }
    pub fn emoji(&self) -> &str {
        &self.emoji
    }
    pub fn token(&self) -> &str {
        &self.token
    }
    pub fn script(&self) -> &str {
        &self.script
    }
    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn script_enabled(&self) -> bool {
        self.script_enabled
    }
}
