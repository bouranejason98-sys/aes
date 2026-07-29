use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeCategory {
    Operational,
    Financial,
    Market,
    Security,
    Technical,
    Strategic,
}

impl std::fmt::Display for KnowledgeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeCategory::Operational => write!(f, "OPERATIONAL"),
            KnowledgeCategory::Financial => write!(f, "FINANCIAL"),
            KnowledgeCategory::Market => write!(f, "MARKET"),
            KnowledgeCategory::Security => write!(f, "SECURITY"),
            KnowledgeCategory::Technical => write!(f, "TECHNICAL"),
            KnowledgeCategory::Strategic => write!(f, "STRATEGIC"),
        }
    }
}
