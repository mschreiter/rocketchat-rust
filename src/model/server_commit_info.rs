use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCommitInfo {
    hash: String,
    date: String,
    author: String,
    subject: String,
    tag: String,
    branch: String,
}

impl ServerCommitInfo {
    pub fn new() -> Self {
        Self {
            hash: String::new(),
            date: String::new(),
            author: String::new(),
            subject: String::new(),
            tag: String::new(),
            branch: String::new(),
        }
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
    pub fn date(&self) -> &str {
        &self.date
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn subject(&self) -> &str {
        &self.subject
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn branch(&self) -> &str {
        &self.branch
    }
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
    pub fn set_date(&mut self, date: String) {
        self.date = date;
    }
    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }
    pub fn set_subject(&mut self, subject: String) {
        self.subject = subject;
    }
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }
    pub fn set_branch(&mut self, branch: String) {
        self.branch = branch;
    }
}
