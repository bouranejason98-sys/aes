use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Event,
    Command,
    Response,
    Notification,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MessageType::Event => write!(f, "EVENT"),
            MessageType::Command => write!(f, "COMMAND"),
            MessageType::Response => write!(f, "RESPONSE"),
            MessageType::Notification => write!(f, "NOTIFICATION"),
        }
    }
}
