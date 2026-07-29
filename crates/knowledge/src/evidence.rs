use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Raw data extracted from a mission or external source.
/// Knowledge is derived FROM evidence, never without it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub mission_id: Option<Uuid>,
    pub content: String,
    pub extracted_at: chrono::DateTime<chrono::Utc>,
}

impl Evidence {
    pub fn new(mission_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id: Some(mission_id),
            content,
            extracted_at: chrono::Utc::now(),
        }
    }

    pub fn standalone(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id: None,
            content,
            extracted_at: chrono::Utc::now(),
        }
    }
}
