use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GoalState {
    Draft,
    Active,
    Paused,
    Completed,
    Archived,
}

impl std::fmt::Display for GoalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalState::Draft => write!(f, "DRAFT"),
            GoalState::Active => write!(f, "ACTIVE"),
            GoalState::Paused => write!(f, "PAUSED"),
            GoalState::Completed => write!(f, "COMPLETED"),
            GoalState::Archived => write!(f, "ARCHIVED"),
        }
    }
}
