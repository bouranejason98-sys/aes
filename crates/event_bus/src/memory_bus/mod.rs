use std::sync::{Arc, Mutex};
use std::panic;
use aes_protocol::{AespEnvelope, Topic};
use crate::publisher::Publisher;
use crate::subscriber::{Subscriber, Handler};
use crate::dispatcher::Dispatcher;

// Subscriptions are stored in a Vec, not a HashMap, so dispatch order
// matches subscription order and is reproducible on every run.
pub struct MemoryBus {
    subscribers: Arc<Mutex<Vec<(String, Vec<Handler>)>>>,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subscribers: Arc::new(Mutex::new(Vec::new())),
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
        for entry in subs.iter_mut() {
            if entry.0 == topic {
                entry.1.push(handler);
                return;
            }
        }
        subs.push((topic.to_string(), vec![handler]));
    }

    fn unsubscribe(&self, _topic: &str) {
        // Implementation for future
    }
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
            let envelope_clone = envelope.clone();
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                handler(&envelope_clone);
            }));

            if let Err(e) = result {
                eprintln!("[MEMORY_BUS] Subscriber panicked: {:?}", e);
            }
        }
    }
}
