use serde::{Serialize, Deserialize};

/// What the goal is actually trying to achieve, stated as a single
/// declarative sentence. Kept separate from `description` (free-form
/// context) so a future Decision Engine has one canonical field to
/// reason over instead of parsing prose out of the description.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    pub statement: String,
}

impl Objective {
    pub fn new(statement: String) -> Self {
        Objective { statement }
    }
}

impl std::fmt::Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)
    }
}
