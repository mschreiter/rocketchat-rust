use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerBuildInfo {
    date: String,
    node_version: String,
    arch: String,
    platform: String,
    os_release: String,
    total_memory: String,
    free_memory: String,
    cpus: i32,
}

impl ServerBuildInfo {
    pub fn new() -> Self {
        Self {
            date: String::new(),
            node_version: String::new(),
            arch: String::new(),
            platform: String::new(),
            os_release: String::new(),
            total_memory: String::new(),
            free_memory: String::new(),
            cpus: 0,
        }
    }

    pub fn date(&self) -> &str {
        &self.date
    }
    pub fn node_version(&self) -> &str {
        &self.node_version
    }
    pub fn arch(&self) -> &str {
        &self.arch
    }
    pub fn platform(&self) -> &str {
        &self.platform
    }
    pub fn os_release(&self) -> &str {
        &self.os_release
    }
    pub fn total_memory(&self) -> &str {
        &self.total_memory
    }
    pub fn free_memory(&self) -> &str {
        &self.free_memory
    }
    pub fn cpus(&self) -> i32 {
        self.cpus
    }
}
