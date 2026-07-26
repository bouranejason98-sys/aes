use aes_protocol::AespEnvelope;

pub trait Publisher {
    fn publish(&self, envelope: AespEnvelope) -> Result<(), Box<dyn std::error::Error>>;
}
