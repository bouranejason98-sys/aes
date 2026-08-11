use std::sync::{Arc, Mutex};

use aes_event_bus::Publisher;
use aes_protocol::create_event;

use crate::command::CreateGoalCommand;
use crate::goal::Goal;
use crate::goal_id::GoalId;
use crate::repository::GoalRepository;
use crate::errors::GoalError;

/// Orchestrates goal creation, mirroring KnowledgeManager's structure:
/// depends on GoalRepository as a trait object, and on Publisher as a
/// trait object -- not Subscriber, since nothing upstream feeds Goals
/// yet. One public entry point (`handle`), not create()/store() split
/// across multiple methods.
pub struct GoalManager {
    repository: Mutex<Box<dyn GoalRepository + Send>>,
    publisher: Arc<dyn Publisher + Send + Sync>,
}

impl GoalManager {
    pub fn new(
        repository: Box<dyn GoalRepository + Send>,
        publisher: Arc<dyn Publisher + Send + Sync>,
    ) -> Self {
        GoalManager {
            repository: Mutex::new(repository),
            publisher,
        }
    }

    pub fn handle(&self, command: CreateGoalCommand) -> Result<GoalId, GoalError> {
        let mut goal = Goal::new(
            command.name,
            command.description,
            command.priority,
            command.objective,
        )?;

        for constraint in command.constraints {
            goal = goal.with_constraint(constraint);
        }

        let id = goal.id;
        let name = goal.name.clone();
        let priority = goal.priority.to_string();

        {
            let mut repo = self.repository.lock().unwrap();
            repo.insert(goal)?;
        }

        let payload = serde_json::json!({
            "goal_id": id.to_string(),
            "name": name,
            "priority": priority,
        });
        let envelope = create_event("goal_manager", "goal.created", payload);
        if let Err(e) = self.publisher.publish(envelope) {
            eprintln!("[GOALS] Failed to publish goal.created: {}", e);
        }

        Ok(id)
    }

    pub fn count(&self) -> usize {
        self.repository.lock().unwrap().count()
    }

    pub fn names(&self) -> Vec<String> {
        self.repository.lock().unwrap().iter().map(|g| g.name.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::priority::Priority;
    use crate::objective::Objective;
    use crate::constraint::Constraint;
    use crate::repository::MemoryGoalRepository;
    use aes_protocol::AespEnvelope;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingPublisher {
        count: AtomicUsize,
    }

    impl Publisher for CountingPublisher {
        fn publish(&self, _envelope: AespEnvelope) -> Result<(), Box<dyn std::error::Error>> {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn test_handle_creates_stores_and_publishes() {
        let repo = Box::new(MemoryGoalRepository::new());
        let publisher_impl = Arc::new(CountingPublisher { count: AtomicUsize::new(0) });
        // Explicit typed binding for the coercion -- Arc::clone(&x) passed
        // directly as an argument doesn't reliably coerce Arc<Concrete> to
        // Arc<dyn Trait>; a typed let does. Same fix as bootstrap.rs.
        let publisher: Arc<dyn Publisher + Send + Sync> = publisher_impl.clone();
        let manager = GoalManager::new(repo, publisher);

        let command = CreateGoalCommand {
            name: "Grow Capital".to_string(),
            description: "Long-term capital growth".to_string(),
            priority: Priority::High,
            objective: Objective::new("Maximize long-term capital growth".to_string()),
            constraints: vec![Constraint::hard("Never violate Constitution".to_string())],
        };

        let id = manager.handle(command).unwrap();

        assert_eq!(manager.count(), 1);
        assert_eq!(publisher_impl.count.load(Ordering::SeqCst), 1);
        assert!(manager.names().contains(&"Grow Capital".to_string()));
        let _ = id;
    }

    #[test]
    fn test_handle_rejects_empty_name() {
        let repo = Box::new(MemoryGoalRepository::new());
        let publisher_impl = Arc::new(CountingPublisher { count: AtomicUsize::new(0) });
        let publisher: Arc<dyn Publisher + Send + Sync> = publisher_impl.clone();
        let manager = GoalManager::new(repo, publisher);

        let command = CreateGoalCommand {
            name: "".to_string(),
            description: "desc".to_string(),
            priority: Priority::Normal,
            objective: Objective::new("obj".to_string()),
            constraints: vec![],
        };

        let result = manager.handle(command);
        assert!(result.is_err());
        assert_eq!(manager.count(), 0);
        assert_eq!(publisher_impl.count.load(Ordering::SeqCst), 0);
    }
}
