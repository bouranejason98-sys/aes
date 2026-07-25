use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
pub struct HealthStatus {
    pub name: String,
    pub version: String,
    pub status: String,
    pub last_heartbeat: String,
    pub startup_time: String,
    pub error_count: u32,
    pub latency_ms: u64,
}

pub struct HealthMonitor;

impl HealthMonitor {
    pub fn new() -> Self {
        HealthMonitor
    }

    pub fn check_service(&self, name: &str, version: &str, status: &str) -> HealthStatus {
        HealthStatus {
            name: name.to_string(),
            version: version.to_string(),
            status: status.to_string(),
            last_heartbeat: Utc::now().to_rfc3339(),
            startup_time: Utc::now().to_rfc3339(),
            error_count: 0,
            latency_ms: 1,
        }
    }
}
