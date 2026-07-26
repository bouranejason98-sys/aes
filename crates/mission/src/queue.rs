use std::collections::VecDeque;
use crate::mission::Mission;

pub struct MissionQueue {
    queue: VecDeque<Mission>,
    max_size: usize,
}

impl MissionQueue {
    pub fn new(max_size: usize) -> Self {
        MissionQueue {
            queue: VecDeque::new(),
            max_size,
        }
    }

    pub fn push(&mut self, mission: Mission) -> Result<(), crate::errors::MissionError> {
        if self.queue.len() >= self.max_size {
            return Err(crate::errors::MissionError::QueueFull);
        }
        self.queue.push_back(mission);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Mission> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
