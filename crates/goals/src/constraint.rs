use serde::{Serialize, Deserialize};

/// Whether a constraint is absolute ("never...") or should be weighed
/// against other factors ("respect..."). A future Decision Engine reads
/// this distinction; Phase 1 only needs to carry it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintKind {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Constraint {
    pub description: String,
    pub kind: ConstraintKind,
}

impl Constraint {
    pub fn hard(description: String) -> Self {
        Constraint { description, kind: ConstraintKind::Hard }
    }

    pub fn soft(description: String) -> Self {
        Constraint { description, kind: ConstraintKind::Soft }
    }
}
