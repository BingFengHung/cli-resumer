use chrono::{DateTime, Local, Utc};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ProviderType {
    Agy,
    Copilot,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::Agy => write!(f, "AGY CLI"),
            ProviderType::Copilot => write!(f, "Copilot CLI"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub title: String,
    pub timestamp: DateTime<Utc>,
    pub workspace_path: Option<PathBuf>,
    pub provider: ProviderType,
}

impl SessionInfo {
    pub fn formatted_time(&self) -> String {
        let local_time: DateTime<Local> = DateTime::from(self.timestamp);
        local_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
