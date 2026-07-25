use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum KernelState {
    Created,
    Booting,
    Initializing,
    Ready,
    Running,
    Stopping,
    Stopped,
    Degraded,
    Recovering,
}

pub struct StateManager {
    current: KernelState,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current: KernelState::Created,
        }
    }

    pub fn transition(&mut self, next: KernelState) {
        self.current = next;
    }

    pub fn get(&self) -> &KernelState {
        &self.current
    }
}
