use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic(pub String);

impl Topic {
    pub fn new(s: &str) -> Self {
        Topic(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    // Simple prefix match for wildcard-like behavior (e.g., "system.*" matches "system.boot")
    pub fn matches(&self, other: &str) -> bool {
        if self.0.ends_with('*') {
            let prefix = &self.0[..self.0.len() - 1];
            other.starts_with(prefix)
        } else {
            self.0 == other
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
