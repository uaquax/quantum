use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitMessage {
    pub target: String,
    pub reply: String,
    pub wait_message: Option<Box<WaitMessage>>,
}
