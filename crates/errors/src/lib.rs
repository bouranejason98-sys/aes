use thiserror::Error;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Configuration load failed: {0}")]
    ConfigLoad(String),
    
    #[error("Service initialization failed: {0}")]
    ServiceInit(String),
    
    #[error("Service start failed: {0}")]
    ServiceStart(String),
    
    #[error("Service shutdown failed: {0}")]
    ServiceShutdown(String),
    
    #[error("State transition invalid: {0} -> {1}")]
    InvalidStateTransition(String, String),
    
    #[error("Dependency missing: {0}")]
    MissingDependency(String),
}
