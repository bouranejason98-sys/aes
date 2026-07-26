use crate::envelope::AespEnvelope;
use crate::message::MessageType;
use crate::correlation::CorrelationId;

pub fn create_response(source: &str, topic: &str, payload: serde_json::Value, correlation_id: CorrelationId) -> AespEnvelope {
    AespEnvelope::new(source, topic, MessageType::Response, payload)
        .with_correlation_id(correlation_id)
}
