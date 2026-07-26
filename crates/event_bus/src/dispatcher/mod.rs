use aes_protocol::AespEnvelope;

pub trait Dispatcher {
    fn dispatch(&self, envelope: &AespEnvelope);
}
