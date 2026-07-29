use serde::{Serialize, Deserialize};

/// A validated confidence score (0.0 to 1.0).
/// Encodes the architectural rule: Invalid confidence cannot exist.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Confidence(f32);

impl Confidence {
    pub fn new(value: f32) -> Result<Self, crate::errors::KnowledgeError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::errors::KnowledgeError::InvalidConfidence(value))
        }
    }

    pub fn high() -> Self { Self(0.9) }
    pub fn medium() -> Self { Self(0.6) }
    pub fn low() -> Self { Self(0.3) }
    
    pub fn value(&self) -> f32 { self.0 }
}
