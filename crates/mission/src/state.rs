use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionState {
    Created,
    Queued,
    Scheduled,
    Running,
    Completed,
    Failed,
    Retrying,
}

impl std::fmt::Display for MissionState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MissionState::Created => write!(f, "CREATED"),
            MissionState::Queued => write!(f, "QUEUED"),
            MissionState::Scheduled => write!(f, "SCHEDULED"),
            MissionState::Running => write!(f, "RUNNING"),
            MissionState::Completed => write!(f, "COMPLETED"),
            MissionState::Failed => write!(f, "FAILED"),
            MissionState::Retrying => write!(f, "RETRYING"),
        }
    }
}
