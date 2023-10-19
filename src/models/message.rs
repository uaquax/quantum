use serde::{Deserialize, Serialize};

use super::wait_message::WaitMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub wait_message: Option<Box<WaitMessage>>,
}
