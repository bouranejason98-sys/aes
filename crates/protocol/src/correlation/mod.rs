use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrelationId(pub Uuid);

impl CorrelationId {
    pub fn new() -> Self {
        CorrelationId(Uuid::new_v4())
    }

    pub fn from_u128(val: u128) -> Self {
        CorrelationId(Uuid::from_u128(val))
    }

    pub fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}
