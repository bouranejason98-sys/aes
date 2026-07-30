use crate::category::KnowledgeCategory;
use crate::source::KnowledgeSource;
use crate::evidence::Evidence;

/// Intent to store a new piece of knowledge. Carries only the raw
/// ingredients -- no repository reference, no event bus reference, no
/// kernel reference. The manager turns this into a validated `Knowledge`
/// via `Knowledge::from_evidence`, which already enforces the
/// evidence-required rule.
#[derive(Debug, Clone)]
pub struct StoreKnowledgeCommand {
    pub title: String,
    pub category: KnowledgeCategory,
    pub source: KnowledgeSource,
    pub evidence: Evidence,
}
