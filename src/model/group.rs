use serde::{Deserialize, Serialize};

use model::room_creator::RoomCreator;
use model::room_type::RoomType;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    id: String,
    name: String,
    topic: String,
    description: String,
    #[serde(rename = "type")]
    room_type: RoomType,
    creator: RoomCreator,
    user_names: Vec<String>,
    msg_count: i32,
    created: String,
    updated: String,
    read_only: bool,
    sys_msgs: bool,
    archived: bool,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            topic: String::new(),
            description: String::new(),
            room_type: RoomType::PrivateGroup,
            creator: RoomCreator::new(),
            user_names: Vec::new(),
            msg_count: 0,
            created: String::new(),
            updated: String::new(),
            read_only: false,
            sys_msgs: false,
            archived: false,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn topic(&self) -> &str {
        &self.topic
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn room_type(&self) -> &RoomType {
        &self.room_type
    }
    pub fn creator(&self) -> &RoomCreator {
        &self.creator
    }
    pub fn user_names(&self) -> &Vec<String> {
        &self.user_names
    }
    pub fn msg_count(&self) -> i32 {
        self.msg_count
    }
    pub fn created(&self) -> &str {
        &self.created
    }
    pub fn updated(&self) -> &str {
        &self.updated
    }
    pub fn read_only(&self) -> bool {
        self.read_only
    }
    pub fn sys_msgs(&self) -> bool {
        self.sys_msgs
    }
    pub fn archived(&self) -> bool {
        self.archived
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_room_type(&mut self, room_type: RoomType) {
        self.room_type = room_type;
    }
    pub fn set_creator(&mut self, creator: RoomCreator) {
        self.creator = creator;
    }
    pub fn set_user_names(&mut self, user_names: Vec<String>) {
        self.user_names = user_names;
    }
    pub fn set_msg_count(&mut self, msg_count: i32) {
        self.msg_count = msg_count;
    }
    pub fn set_created(&mut self, created: String) {
        self.created = created;
    }
    pub fn set_updated(&mut self, updated: String) {
        self.updated = updated;
    }
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
    pub fn set_sys_msgs(&mut self, sys_msgs: bool) {
        self.sys_msgs = sys_msgs;
    }
    pub fn set_archived(&mut self, archived: bool) {
        self.archived = archived;
    }
}
