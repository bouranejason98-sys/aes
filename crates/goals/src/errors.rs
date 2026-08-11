use std::fmt;

#[derive(Debug)]
pub enum GoalError {
    InvalidStateTransition { from: String, to: String },
    EmptyName,
}

impl fmt::Display for GoalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoalError::InvalidStateTransition { from, to } => {
                write!(f, "Invalid goal state transition: {} -> {}", from, to)
            }
            GoalError::EmptyName => write!(f, "Goal name cannot be empty"),
        }
    }
}

impl std::error::Error for GoalError {}
