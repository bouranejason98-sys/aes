use std::sync::{Arc, Mutex};
use uuid::Uuid;

use aes_event_bus::Publisher;
use aes_protocol::create_event;

use crate::command::StoreKnowledgeCommand;
use crate::knowledge::Knowledge;
use crate::repository::KnowledgeRepository;
use crate::errors::KnowledgeError;

pub struct KnowledgeManager {
    repository: Mutex<Box<dyn KnowledgeRepository + Send>>,
    publisher: Arc<dyn Publisher + Send + Sync>,
}

impl KnowledgeManager {
    pub fn new(
        repository: Box<dyn KnowledgeRepository + Send>,
        publisher: Arc<dyn Publisher + Send + Sync>,
    ) -> Self {
        KnowledgeManager {
            repository: Mutex::new(repository),
            publisher,
        }
    }

    pub fn handle(&self, command: StoreKnowledgeCommand) -> Result<Uuid, KnowledgeError> {
        let knowledge = Knowledge::from_evidence(
            command.title,
            command.category,
            command.source,
            command.evidence,
        )?;

        let id = knowledge.id;
        let category = knowledge.category.to_string();
        let title = knowledge.title.clone();

        {
            let mut repo = self.repository.lock().unwrap();
            repo.insert(knowledge)?;
        }

        let payload = serde_json::json!({
            "knowledge_id": id.to_string(),
            "category": category,
            "title": title,
        });
        let envelope = create_event("knowledge_manager", "knowledge.created", payload);
        if let Err(e) = self.publisher.publish(envelope) {
            eprintln!("[KNOWLEDGE] Failed to publish knowledge.created: {}", e);
        }

        Ok(id)
    }

    pub fn count(&self) -> usize {
        self.repository.lock().unwrap().count()
    }

    pub fn titles(&self) -> Vec<String> {
        self.repository.lock().unwrap().iter().map(|k| k.title.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::KnowledgeCategory;
    use crate::source::KnowledgeSource;
    use crate::evidence::Evidence;
    use crate::repository::MemoryKnowledgeRepository;
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
    fn test_handle_stores_and_publishes() {
        let repo = Box::new(MemoryKnowledgeRepository::new());
        let publisher_impl = Arc::new(CountingPublisher { count: AtomicUsize::new(0) });
        // Explicit typed binding, same fix as bootstrap.rs -- see comment
        // there for why this can't just be `publisher_impl.clone()` passed
        // directly as the argument.
        let publisher: Arc<dyn Publisher + Send + Sync> = publisher_impl.clone();
        let manager = KnowledgeManager::new(repo, publisher);

        let evidence = Evidence::standalone("test evidence".to_string());
        let command = StoreKnowledgeCommand {
            title: "Test".to_string(),
            category: KnowledgeCategory::Technical,
            source: KnowledgeSource::ManualInput,
            evidence,
        };

        let id = manager.handle(command).unwrap();

        assert_eq!(manager.count(), 1);
        assert_eq!(publisher_impl.count.load(Ordering::SeqCst), 1);
        assert!(manager.titles().contains(&"Test".to_string()));
        let _ = id;
    }
}
