use std::sync::{Arc, Mutex};
use aes_protocol::create_event;
use aes_event_bus::{MemoryBus, Publisher};
use crate::mission::Mission;
use crate::queue::MissionQueue;
use crate::state::MissionState;

pub struct MissionScheduler {
    queue: Arc<Mutex<MissionQueue>>,
    event_bus: Arc<MemoryBus>,
}

impl MissionScheduler {
    pub fn new(event_bus: Arc<MemoryBus>) -> Self {
        MissionScheduler {
            queue: Arc::new(Mutex::new(MissionQueue::new(1000))),
            event_bus,
        }
    }

    pub fn enqueue(&self, mut mission: Mission) -> Result<(), crate::errors::MissionError> {
        mission.transition_to(MissionState::Queued)?;

        let payload = serde_json::json!({
            "mission_id": mission.id.to_string(),
            "name": mission.name,
            "priority": mission.priority.to_string()
        });
        let envelope = create_event("scheduler", "mission.created", payload);
        if let Err(e) = self.event_bus.publish(envelope) {
            eprintln!("[SCHEDULER] Failed to publish MissionCreated: {}", e);
        }

        let payload = serde_json::json!({
            "mission_id": mission.id.to_string(),
            "state": mission.state.to_string()
        });
        let envelope = create_event("scheduler", "mission.queued", payload);
        if let Err(e) = self.event_bus.publish(envelope) {
            eprintln!("[SCHEDULER] Failed to publish MissionQueued: {}", e);
        }

        let mut queue = self.queue.lock().unwrap();
        queue.push(mission)
    }

    pub fn dequeue(&mut self) -> Option<Mission> {
        let popped = {
            let mut queue = self.queue.lock().unwrap();
            queue.pop()
        };

        let mut mission = popped?;

        if let Err(e) = mission.transition_to(MissionState::Scheduled) {
            eprintln!("[SCHEDULER] Failed to schedule mission: {}", e);
            return Some(mission);
        }

        let payload = serde_json::json!({
            "mission_id": mission.id.to_string(),
            "state": mission.state.to_string()
        });
        let envelope = create_event("scheduler", "mission.scheduled", payload);
        if let Err(e) = self.event_bus.publish(envelope) {
            eprintln!("[SCHEDULER] Failed to publish MissionScheduled: {}", e);
        }

        Some(mission)
    }

    pub fn get_queue_length(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}
