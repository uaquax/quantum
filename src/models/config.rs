use std::fs;

use serde::{Deserialize, Serialize};

use super::{message::Message, wait_message::WaitMessage};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub back_messages: Vec<WaitMessage>,
    pub messages: Vec<Message>,
    pub target_id: i64,
    pub target_username: String,
}

impl Config {
    pub fn new() -> Self {
        let parsed_json: Config =
            serde_json::from_str(fs::read_to_string("config.json").unwrap().as_str()).unwrap();

        Self {
            target_id: parsed_json.target_id,
            target_username: parsed_json.target_username,
            messages: parsed_json.messages,
            back_messages: parsed_json.back_messages,
        }
    }
}
