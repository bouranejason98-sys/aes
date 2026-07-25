use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use crate::correlation::CorrelationId;
use crate::message::MessageType;
use crate::topic::Topic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AespEnvelope {
    pub message_id: Uuid,
    pub correlation_id: CorrelationId,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub target: Option<String>,
    pub topic: Topic,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub metadata: std::collections::HashMap<String, String>,
}

impl AespEnvelope {
    pub fn new(
        source: &str,
        topic: &str,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        AespEnvelope {
            message_id: Uuid::new_v4(),
            correlation_id: CorrelationId::new(),
            timestamp: Utc::now(),
            source: source.to_string(),
            target: None,
            topic: Topic::new(topic),
            message_type,
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_correlation_id(mut self, id: CorrelationId) -> Self {
        self.correlation_id = id;
        self
    }
}
