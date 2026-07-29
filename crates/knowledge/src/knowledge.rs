use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;

use crate::confidence::Confidence;
use crate::category::KnowledgeCategory;
use crate::source::KnowledgeSource;
use crate::evidence::Evidence;
use crate::errors::KnowledgeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: Uuid,
    pub title: String,
    pub category: KnowledgeCategory,
    pub confidence: Confidence,
    pub source: KnowledgeSource,
    pub evidence: Vec<Evidence>,
    pub created_at: DateTime<Utc>,
}

impl Knowledge {
    /// Architectural Rule: Knowledge MUST be created from Evidence.
    pub fn from_evidence(
        title: String,
        category: KnowledgeCategory,
        source: KnowledgeSource,
        evidence: Evidence,
    ) -> Result<Self, KnowledgeError> {
        Self::from_evidence_list(title, category, source, vec![evidence])
    }

    pub fn from_evidence_list(
        title: String,
        category: KnowledgeCategory,
        source: KnowledgeSource,
        evidence: Vec<Evidence>,
    ) -> Result<Self, KnowledgeError> {
        if evidence.is_empty() {
            return Err(KnowledgeError::MissingEvidence);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            title,
            category,
            confidence: Confidence::medium(), // Default, can be refined later
            source,
            evidence,
            created_at: Utc::now(),
        })
    }
}
