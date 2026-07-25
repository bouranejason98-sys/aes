use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppMeta,
    pub logging: LoggingConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppMeta {
    pub name: String,
    pub version: String,
    pub mode: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
}

pub fn load(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}
