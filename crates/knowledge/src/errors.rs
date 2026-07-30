use std::fmt;

#[derive(Debug)]
pub enum KnowledgeError {
    InvalidConfidence(f32),
    MissingEvidence,
    InvalidCategory(String),
    IdParseError(String),
    MissingField(String),
}

impl fmt::Display for KnowledgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KnowledgeError::InvalidConfidence(val) => write!(f, "Confidence must be 0.0-1.0, got {}", val),
            KnowledgeError::MissingEvidence => write!(f, "Knowledge cannot be created without evidence"),
            KnowledgeError::InvalidCategory(cat) => write!(f, "Invalid category: {}", cat),
            KnowledgeError::IdParseError(id) => write!(f, "Failed to parse ID: {}", id),
            KnowledgeError::MissingField(field) => write!(f, "Missing required field in event payload: {}", field),
        }
    }
}

impl std::error::Error for KnowledgeError {}
