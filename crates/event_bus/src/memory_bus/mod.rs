use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::panic;
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

        // CRITICAL FIX: Catch panics in individual handlers
        for handler in matched_handlers {
            let envelope_clone = envelope.clone(); // Clone envelope for safe passing
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                handler(&envelope_clone);
            }));

            if let Err(e) = result {
                // Log the panic but DO NOT stop the loop
                // In a real system, we would log this to a monitoring system
                eprintln!("[MEMORY_BUS] Subscriber panicked: {:?}", e);
            }
        }
    }
}
