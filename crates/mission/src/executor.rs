use std::sync::Arc;
use aes_protocol::create_event;
use aes_event_bus::{MemoryBus, Publisher};
use crate::mission::Mission;
use crate::state::MissionState;

pub struct MissionExecutor {
    event_bus: Arc<MemoryBus>,
}

impl MissionExecutor {
    pub fn new(event_bus: Arc<MemoryBus>) -> Self {
        MissionExecutor { event_bus }
    }

    pub fn execute(&self, mut mission: Mission) -> Result<(), crate::errors::MissionError> {
        mission.transition_to(MissionState::Running)?;

        let payload = serde_json::json!({
            "mission_id": mission.id.to_string(),
            "name": mission.name,
            "state": mission.state.to_string()
        });
        let envelope = create_event("executor", "mission.started", payload);
        if let Err(e) = self.event_bus.publish(envelope) {
            eprintln!("[EXECUTOR] Failed to publish MissionStarted: {}", e);
        }

        let result = self.run_mission_logic(&mission);

        match result {
            Ok(report) => {
                mission.transition_to(MissionState::Completed)?;

                let payload = serde_json::json!({
                    "mission_id": mission.id.to_string(),
                    "name": mission.name,
                    "state": mission.state.to_string(),
                    "report": report
                });
                let envelope = create_event("executor", "mission.completed", payload);
                if let Err(e) = self.event_bus.publish(envelope) {
                    eprintln!("[EXECUTOR] Failed to publish MissionCompleted: {}", e);
                }
                Ok(())
            }
            Err(e) => {
                mission.transition_to(MissionState::Failed)?;

                let payload = serde_json::json!({
                    "mission_id": mission.id.to_string(),
                    "name": mission.name,
                    "error": e.to_string()
                });
                let envelope = create_event("executor", "mission.failed", payload);
                if let Err(e) = self.event_bus.publish(envelope) {
                    eprintln!("[EXECUTOR] Failed to publish MissionFailed: {}", e);
                }
                Err(e)
            }
        }
    }

    fn run_mission_logic(&self, mission: &Mission) -> Result<String, crate::errors::MissionError> {
        if mission.name == "SystemHealthCheck" {
            let report = format!("Health Check Report for {}: OK - All systems nominal.", mission.name);
            Ok(report)
        } else {
            Err(crate::errors::MissionError::ExecutionFailed(format!("Unknown mission: {}", mission.name)))
        }
    }
}
