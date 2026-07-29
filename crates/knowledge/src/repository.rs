use crate::knowledge::Knowledge;
use crate::errors::KnowledgeError;

/// Contract for storing and retrieving Knowledge.
/// This trait allows the implementation to evolve (e.g., to a database) without changing callers.
pub trait KnowledgeRepository {
    /// Store a new knowledge record.
    fn insert(&mut self, knowledge: Knowledge) -> Result<(), KnowledgeError>;

    /// Retrieve a knowledge record by ID.
    fn get(&self, id: &uuid::Uuid) -> Option<&Knowledge>;

    /// Check if a knowledge record exists by ID.
    fn contains(&self, id: &uuid::Uuid) -> bool;

    /// Get the total number of stored records.
    fn count(&self) -> usize;

    /// Iterate over all stored knowledge records.
    fn iter(&self) -> Box<dyn Iterator<Item = &Knowledge> + '_>;
}

use std::collections::HashMap;

/// In-memory implementation of KnowledgeRepository.
/// Suitable for testing and initial development.
pub struct MemoryKnowledgeRepository {
    storage: HashMap<uuid::Uuid, Knowledge>,
}

impl MemoryKnowledgeRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl Default for MemoryKnowledgeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl KnowledgeRepository for MemoryKnowledgeRepository {
    fn insert(&mut self, knowledge: Knowledge) -> Result<(), KnowledgeError> {
        let id = knowledge.id;
        // In a real system, we might check for duplicates here
        self.storage.insert(id, knowledge);
        Ok(())
    }

    fn get(&self, id: &uuid::Uuid) -> Option<&Knowledge> {
        self.storage.get(id)
    }

    fn contains(&self, id: &uuid::Uuid) -> bool {
        self.storage.contains_key(id)
    }

    fn count(&self) -> usize {
        self.storage.len()
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Knowledge> + '_> {
        Box::new(self.storage.values())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::category::KnowledgeCategory;
    use crate::source::KnowledgeSource;
    use crate::evidence::Evidence;

    fn create_test_knowledge(title: &str) -> Knowledge {
        let evidence = Evidence::standalone("Test evidence".to_string());
        Knowledge::from_evidence(
            title.to_string(),
            KnowledgeCategory::Technical,
            KnowledgeSource::ManualInput,
            evidence,
        ).unwrap()
    }

    #[test]
    fn test_insert_and_retrieve() {
        let mut repo = MemoryKnowledgeRepository::new();
        let knowledge = create_test_knowledge("Test Knowledge");
        let id = knowledge.id;

        repo.insert(knowledge).unwrap();

        assert_eq!(repo.count(), 1);
        assert!(repo.contains(&id));
        
        let retrieved = repo.get(&id).unwrap();
        assert_eq!(retrieved.title, "Test Knowledge");
    }

    #[test]
    fn test_empty_repository() {
        let repo = MemoryKnowledgeRepository::new();
        assert_eq!(repo.count(), 0);
        assert!(!repo.contains(&uuid::Uuid::new_v4()));
        assert!(repo.iter().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_duplicate_insert() {
        let mut repo = MemoryKnowledgeRepository::new();
        let mut knowledge = create_test_knowledge("Original");
        let id = knowledge.id;

        repo.insert(knowledge.clone()).unwrap();
        assert_eq!(repo.count(), 1);

        // Update title and insert again (should overwrite)
        knowledge.title = "Updated".to_string();
        repo.insert(knowledge).unwrap();
        
        assert_eq!(repo.count(), 1); // Count remains 1
        let retrieved = repo.get(&id).unwrap();
        assert_eq!(retrieved.title, "Updated");
    }

    #[test]
    fn test_iter() {
        let mut repo = MemoryKnowledgeRepository::new();
        repo.insert(create_test_knowledge("K1")).unwrap();
        repo.insert(create_test_knowledge("K2")).unwrap();
        repo.insert(create_test_knowledge("K3")).unwrap();

        let titles: Vec<String> = repo.iter().map(|k| k.title.clone()).collect();
        assert_eq!(titles.len(), 3);
        assert!(titles.contains(&"K1".to_string()));
        assert!(titles.contains(&"K2".to_string()));
        assert!(titles.contains(&"K3".to_string()));
    }
}
