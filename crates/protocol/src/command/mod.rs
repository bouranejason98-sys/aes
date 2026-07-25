use crate::envelope::AespEnvelope;
use crate::message::MessageType;

pub fn create_command(source: &str, target: &str, topic: &str, payload: serde_json::Value) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Command, payload)
        .with_target(target)
}
