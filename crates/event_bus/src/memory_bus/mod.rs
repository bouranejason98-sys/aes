use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use aes_protocol::{AespEnvelope, Topic};
use crate::publisher::Publisher;
use crate::subscriber::{Subscriber, Handler};
use crate::dispatcher::Dispatcher;

pub struct MemoryBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<Handler>>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Publisher for MemoryBus {
    fn publish(&self, envelope: AespEnvelope) -> Result<(), Box<dyn std::error::Error>> {
        self.dispatch(&envelope);
        Ok(())
    }
}

impl Subscriber for MemoryBus {
    fn subscribe(&self, topic: &str, handler: Handler) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    fn unsubscribe(&self, _topic: &str) {}
}

impl Dispatcher for MemoryBus {
    fn dispatch(&self, envelope: &AespEnvelope) {
        let subs = self.subscribers.lock().unwrap();
        let topic_str = envelope.topic.as_str();
        let mut matched_handlers = Vec::new();

        for (sub_topic, handlers) in subs.iter() {
            let t = Topic::new(sub_topic);
            if t.matches(topic_str) {
                matched_handlers.extend(handlers.iter().cloned());
            }
        }

        drop(subs);

        for handler in matched_handlers {
            handler(envelope);
        }
    }
}
