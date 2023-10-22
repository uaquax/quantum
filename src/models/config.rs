use std::fs;

use serde::{Deserialize, Serialize};

use super::{message::Message, wait_message::WaitMessage};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub target_id: i64,
    pub dialog_keyword: String,
    pub dialogs_channel: i64,
    pub back_messages: Vec<WaitMessage>,
    pub messages: Vec<Message>,
}

impl Config {
    pub fn new() -> Self {
        let parsed_json: Config =
            serde_json::from_str(fs::read_to_string("config.json").unwrap().as_str()).unwrap();

        Self {
            target_id: parsed_json.target_id,
            dialog_keyword: parsed_json.dialog_keyword,
            dialogs_channel: parsed_json.dialogs_channel,
            messages: parsed_json.messages,
            back_messages: parsed_json.back_messages,
        }
    }
}
