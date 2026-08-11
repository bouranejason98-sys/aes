use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::goal_id::GoalId;
use crate::priority::Priority;
use crate::objective::Objective;
use crate::constraint::Constraint;
use crate::lifecycle::GoalState;
use crate::errors::GoalError;

/// A long-lived objective. Distinct from a Mission: a Mission is one unit
/// of temporary work; a Goal is the reason that work gets generated, and
/// may outlive many missions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub name: String,
    pub description: String,
    pub priority: Priority,
    pub objective: Objective,
    pub constraints: Vec<Constraint>,
    pub state: GoalState,
    pub created_at: DateTime<Utc>,
}

impl Goal {
    pub fn new(
        name: String,
        description: String,
        priority: Priority,
        objective: Objective,
    ) -> Result<Self, GoalError> {
        if name.trim().is_empty() {
            return Err(GoalError::EmptyName);
        }

        Ok(Goal {
            id: GoalId::new(),
            name,
            description,
            priority,
            objective,
            constraints: Vec::new(),
            state: GoalState::Draft,
            created_at: Utc::now(),
        })
    }

    pub fn with_constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Validated lifecycle transition, mirroring Mission::transition_to
    /// in aes-mission: state only ever changes through this method, and
    /// invalid transitions are rejected rather than silently applied.
    pub fn transition_to(&mut self, next: GoalState) -> Result<(), GoalError> {
        use GoalState::*;

        let valid = matches!(
            (self.state, next),
            (Draft, Active)
                | (Draft, Archived)
                | (Active, Paused)
                | (Active, Completed)
                | (Paused, Active)
                | (Paused, Archived)
                | (Completed, Archived)
        );

        if valid {
            self.state = next;
            Ok(())
        } else {
            Err(GoalError::InvalidStateTransition {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            })
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, GoalState::Active)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_objective() -> Objective {
        Objective::new("Maximize long-term capital growth".to_string())
    }

    #[test]
    fn test_goal_creation_valid() {
        let goal = Goal::new(
            "Grow Capital".to_string(),
            "Long-term capital growth objective".to_string(),
            Priority::High,
            sample_objective(),
        ).unwrap();

        assert_eq!(goal.name, "Grow Capital");
        assert_eq!(goal.state, GoalState::Draft);
        assert!(goal.constraints.is_empty());
    }

    #[test]
    fn test_goal_creation_empty_name_fails() {
        let result = Goal::new(
            "".to_string(),
            "desc".to_string(),
            Priority::Normal,
            sample_objective(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_with_constraint_builder() {
        let goal = Goal::new(
            "Grow Capital".to_string(),
            "desc".to_string(),
            Priority::High,
            sample_objective(),
        )
        .unwrap()
        .with_constraint(Constraint::hard("Never violate Constitution".to_string()))
        .with_constraint(Constraint::soft("Respect mission deadlines".to_string()));

        assert_eq!(goal.constraints.len(), 2);
    }

    #[test]
    fn test_valid_lifecycle_sequence() {
        let mut goal = Goal::new(
            "Grow Capital".to_string(),
            "desc".to_string(),
            Priority::High,
            sample_objective(),
        ).unwrap();

        goal.transition_to(GoalState::Active).unwrap();
        assert!(goal.is_active());

        goal.transition_to(GoalState::Paused).unwrap();
        assert!(!goal.is_active());

        goal.transition_to(GoalState::Active).unwrap();
        goal.transition_to(GoalState::Completed).unwrap();
        goal.transition_to(GoalState::Archived).unwrap();

        assert_eq!(goal.state, GoalState::Archived);
    }

    #[test]
    fn test_invalid_lifecycle_transition_rejected() {
        let mut goal = Goal::new(
            "Grow Capital".to_string(),
            "desc".to_string(),
            Priority::High,
            sample_objective(),
        ).unwrap();

        // Draft -> Completed skips Active; must be rejected, and state
        // must remain unchanged after a rejected transition.
        let result = goal.transition_to(GoalState::Completed);
        assert!(result.is_err());
        assert_eq!(goal.state, GoalState::Draft);
    }
}
