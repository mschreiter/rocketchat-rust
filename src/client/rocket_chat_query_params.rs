use std::collections::HashMap;

pub struct RocketChatQueryParams {
    pub query_params: HashMap<String, String>,
}

impl RocketChatQueryParams {
    pub fn new() -> Self {
        Self { query_params: HashMap::new() }
    }

    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    // TODO
}
