use serde::{Deserialize, Serialize};

use model::server_build_info::ServerBuildInfo;
use model::server_commit_info::ServerCommitInfo;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    version: String,
    build_info: ServerBuildInfo,
    commit_info: ServerCommitInfo,
}

impl ServerInfo {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            build_info: ServerBuildInfo::new(),
            commit_info: ServerCommitInfo::new(),
        }
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn build_info(&self) -> &ServerBuildInfo {
        &self.build_info
    }
    pub fn commit_info(&self) -> &ServerCommitInfo {
        &self.commit_info
    }
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    pub fn set_build_info(&mut self, build_info: ServerBuildInfo) {
        self.build_info = build_info;
    }
    pub fn set_commit_info(&mut self, commit_info: ServerCommitInfo) {
        self.commit_info = commit_info;
    }
}
