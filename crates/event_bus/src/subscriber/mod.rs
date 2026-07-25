use aes_protocol::AespEnvelope;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(&AespEnvelope) + Send + Sync>;

pub trait Subscriber {
    fn subscribe(&self, topic: &str, handler: Handler);
    fn unsubscribe(&self, topic: &str);
}
