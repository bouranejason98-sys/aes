use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Event {
    pub id: String,
    pub topic: String,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

pub type EventHandler = Box<dyn Fn(&Event) + Send + Sync>;

pub struct EventBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<EventHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, topic: &str, handler: EventHandler) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }

    pub fn publish(&self, event: Event) {
        let subs = self.subscribers.lock().unwrap();
        if let Some(handlers) = subs.get(&event.topic) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}
