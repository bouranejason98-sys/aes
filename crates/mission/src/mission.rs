use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::state::MissionState;
use crate::priority::Priority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: Uuid,
    pub name: String,
    pub priority: Priority,
    pub state: MissionState,
    pub created_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

impl Mission {
    pub fn new(name: &str, priority: Priority, payload: serde_json::Value) -> Self {
        Mission {
            id: Uuid::new_v4(),
            name: name.to_string(),
            priority,
            state: MissionState::Created,
            created_at: Utc::now(),
            payload,
        }
    }

    pub fn transition_to(&mut self, next: MissionState) -> Result<(), crate::errors::MissionError> {
        // State Machine Validation
        match (self.state, next) {
            (MissionState::Created, MissionState::Queued) => {
                self.state = MissionState::Queued;
                Ok(())
            }
            (MissionState::Queued, MissionState::Scheduled) => {
                self.state = MissionState::Scheduled;
                Ok(())
            }
            (MissionState::Scheduled, MissionState::Running) => {
                self.state = MissionState::Running;
                Ok(())
            }
            (MissionState::Running, MissionState::Completed) => {
                self.state = MissionState::Completed;
                Ok(())
            }
            (MissionState::Running, MissionState::Failed) => {
                self.state = MissionState::Failed;
                Ok(())
            }
            (MissionState::Failed, MissionState::Retrying) => {
                self.state = MissionState::Retrying;
                Ok(())
            }
            (MissionState::Retrying, MissionState::Scheduled) => {
                self.state = MissionState::Scheduled;
                Ok(())
            }
            // Invalid transitions
            _ => Err(crate::errors::MissionError::InvalidStateTransition {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            }),
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, MissionState::Completed | MissionState::Failed)
    }
}
