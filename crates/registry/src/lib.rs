use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ServiceInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub status: &'static str,
}

pub struct Registry {
    services: HashMap<String, ServiceInfo>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &'static str, version: &'static str) {
        self.services.insert(
            name.to_string(),
            ServiceInfo {
                name,
                version,
                status: "INITIALIZING",
            },
        );
    }

    pub fn update_status(&mut self, name: &str, status: &'static str) {
        if let Some(info) = self.services.get_mut(name) {
            info.status = status;
        }
    }

    pub fn list(&self) -> Vec<&ServiceInfo> {
        self.services.values().collect()
    }
}
