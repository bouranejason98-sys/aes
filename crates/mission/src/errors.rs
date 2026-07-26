use thiserror::Error;

#[derive(Error, Debug)]
pub enum MissionError {
    #[error("Invalid state transition: {from} -> {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Mission not found: {0}")]
    MissionNotFound(String),

    #[error("Queue is full")]
    QueueFull,

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}
