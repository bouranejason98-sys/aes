use crate::priority::Priority;
use crate::objective::Objective;
use crate::constraint::Constraint;

/// Intent to create a new goal. Mirrors StoreKnowledgeCommand's shape in
/// aes-knowledge: carries only raw ingredients, no repository or event
/// bus reference. Goal::new does the actual validation.
#[derive(Debug, Clone)]
pub struct CreateGoalCommand {
    pub name: String,
    pub description: String,
    pub priority: Priority,
    pub objective: Objective,
    pub constraints: Vec<Constraint>,
}
