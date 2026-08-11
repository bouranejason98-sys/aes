use std::collections::HashMap;
use crate::goal::Goal;
use crate::goal_id::GoalId;
use crate::errors::GoalError;

/// Contract for storing and retrieving Goals. Mirrors KnowledgeRepository
/// in aes-knowledge exactly, keyed on GoalId (a wrapper type) rather than
/// a raw Uuid, because that's what Goal.id actually is.
pub trait GoalRepository {
    fn insert(&mut self, goal: Goal) -> Result<(), GoalError>;
    fn get(&self, id: &GoalId) -> Option<&Goal>;
    fn contains(&self, id: &GoalId) -> bool;
    fn count(&self) -> usize;
    fn iter(&self) -> Box<dyn Iterator<Item = &Goal> + '_>;
}

pub struct MemoryGoalRepository {
    storage: HashMap<GoalId, Goal>,
}

impl MemoryGoalRepository {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }
}

impl Default for MemoryGoalRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GoalRepository for MemoryGoalRepository {
    fn insert(&mut self, goal: Goal) -> Result<(), GoalError> {
        let id = goal.id;
        self.storage.insert(id, goal);
        Ok(())
    }

    fn get(&self, id: &GoalId) -> Option<&Goal> {
        self.storage.get(id)
    }

    fn contains(&self, id: &GoalId) -> bool {
        self.storage.contains_key(id)
    }

    fn count(&self) -> usize {
        self.storage.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Goal> + '_> {
        Box::new(self.storage.values())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::priority::Priority;
    use crate::objective::Objective;

    fn create_test_goal(name: &str) -> Goal {
        Goal::new(
            name.to_string(),
            "Test description".to_string(),
            Priority::Normal,
            Objective::new("Test objective".to_string()),
        ).unwrap()
    }

    #[test]
    fn test_insert_and_retrieve() {
        let mut repo = MemoryGoalRepository::new();
        let goal = create_test_goal("Grow Capital");
        let id = goal.id;

        repo.insert(goal).unwrap();

        assert_eq!(repo.count(), 1);
        assert!(repo.contains(&id));

        let retrieved = repo.get(&id).unwrap();
        assert_eq!(retrieved.name, "Grow Capital");
    }

    #[test]
    fn test_empty_repository() {
        let repo = MemoryGoalRepository::new();
        assert_eq!(repo.count(), 0);
        assert!(!repo.contains(&GoalId::new()));
        assert!(repo.iter().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_duplicate_insert_overwrites() {
        let mut repo = MemoryGoalRepository::new();
        let mut goal = create_test_goal("Original");
        let id = goal.id;

        repo.insert(goal.clone()).unwrap();
        assert_eq!(repo.count(), 1);

        goal.name = "Updated".to_string();
        repo.insert(goal).unwrap();

        assert_eq!(repo.count(), 1);
        let retrieved = repo.get(&id).unwrap();
        assert_eq!(retrieved.name, "Updated");
    }

    #[test]
    fn test_iter() {
        let mut repo = MemoryGoalRepository::new();
        repo.insert(create_test_goal("G1")).unwrap();
        repo.insert(create_test_goal("G2")).unwrap();
        repo.insert(create_test_goal("G3")).unwrap();

        let names: Vec<String> = repo.iter().map(|g| g.name.clone()).collect();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"G1".to_string()));
        assert!(names.contains(&"G2".to_string()));
        assert!(names.contains(&"G3".to_string()));
    }
}
