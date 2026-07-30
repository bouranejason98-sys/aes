use std::sync::Arc;
use uuid::Uuid;

use aes_event_bus::{MemoryBus, Subscriber};
use aes_protocol::AespEnvelope;

use crate::category::KnowledgeCategory;
use crate::source::KnowledgeSource;
use crate::evidence::Evidence;
use crate::command::StoreKnowledgeCommand;
use crate::errors::KnowledgeError;
use crate::manager::KnowledgeManager;

pub fn handle_mission_completed(env: &AespEnvelope) -> Result<StoreKnowledgeCommand, KnowledgeError> {
    let mission_id_str = env.payload.get("mission_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| KnowledgeError::MissingField("mission_id".to_string()))?;

    let mission_id = Uuid::parse_str(mission_id_str)
        .map_err(|e| KnowledgeError::IdParseError(e.to_string()))?;

    let mission_name = env.payload.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown mission");

    let report = env.payload.get("report")
        .and_then(|v| v.as_str())
        .unwrap_or("no report provided");

    let evidence = Evidence::new(mission_id, report.to_string());

    Ok(StoreKnowledgeCommand {
        title: format!("Mission completed: {}", mission_name),
        category: KnowledgeCategory::Operational,
        source: KnowledgeSource::MissionExecution,
        evidence,
    })
}

pub fn register(event_bus: &Arc<MemoryBus>, manager: Arc<KnowledgeManager>) {
    event_bus.subscribe("mission.completed", Arc::new(move |env: &AespEnvelope| {
        match handle_mission_completed(env) {
            Ok(command) => {
                if let Err(e) = manager.handle(command) {
                    eprintln!("[KNOWLEDGE] Failed to store knowledge: {}", e);
                }
            }
            Err(e) => {
                eprintln!("[KNOWLEDGE] Failed to extract knowledge from event: {}", e);
            }
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_protocol::create_event;

    #[test]
    fn test_handle_mission_completed_valid_payload() {
        let payload = serde_json::json!({
            "mission_id": Uuid::new_v4().to_string(),
            "name": "SystemHealthCheck",
            "state": "COMPLETED",
            "report": "All systems nominal"
        });
        let env = create_event("executor", "mission.completed", payload);

        let command = handle_mission_completed(&env).unwrap();
        assert_eq!(command.title, "Mission completed: SystemHealthCheck");
        assert_eq!(command.evidence.content, "All systems nominal");
    }

    #[test]
    fn test_handle_mission_completed_missing_mission_id() {
        let payload = serde_json::json!({ "name": "X" });
        let env = create_event("executor", "mission.completed", payload);

        let result = handle_mission_completed(&env);
        assert!(result.is_err());
    }

    #[test]
    fn test_integration_publish_mission_completed_creates_knowledge() {
        use crate::repository::MemoryKnowledgeRepository;
        use aes_event_bus::Publisher;

        let event_bus = Arc::new(MemoryBus::new());
        // Same coercion fix as bootstrap.rs and manager.rs's test.
        let publisher: Arc<dyn Publisher + Send + Sync> = event_bus.clone();
        let manager = Arc::new(KnowledgeManager::new(
            Box::new(MemoryKnowledgeRepository::new()),
            publisher,
        ));

        register(&event_bus, Arc::clone(&manager));

        let payload = serde_json::json!({
            "mission_id": Uuid::new_v4().to_string(),
            "name": "SystemHealthCheck",
            "report": "All systems nominal"
        });
        let env = create_event("executor", "mission.completed", payload);

        event_bus.publish(env).unwrap();

        assert_eq!(manager.count(), 1);
    }
}
