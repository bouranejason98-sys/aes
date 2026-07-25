use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic(pub String);

impl Topic {
    pub fn new(s: &str) -> Self {
        Topic(s.to_string())
    }

    pub fn matches(&self, other: &str) -> bool {
        // Simple wildcard support: "service.*" matches "service.health"
        if self.0.contains('*') {
            let pattern = self.0.replace('*', ".*");
            let regex = regex::Regex::new(&format!("^{}$", pattern)).unwrap();
            regex.is_match(other)
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
