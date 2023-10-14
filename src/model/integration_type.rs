use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum IntegrationType {
    #[default]
    #[serde(rename = "webhook-outgoing")]
    OutgoingWebhook,
    #[serde(rename = "webhook-incoming")]
    IncomingWebhook,
}
