use crate::envelope::AespEnvelope;
use crate::message::MessageType;

pub fn create_event(source: &str, topic: &str, payload: serde_json::Value) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Event, payload)
}
