use anyhow::Result;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_target: String,
    pub default_select: bool,
    pub page_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_target: "agy".to_string(),
            default_select: false,
            page_size: 10,
        }
    }
}

impl Config {
    pub fn path() -> Option<PathBuf> {
        config_dir().map(|d| d.join("cli-resumer").join("config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::path() {
            if path.exists() {
                if let Ok(file) = File::open(&path) {
                    if let Ok(cfg) = serde_json::from_reader(file) {
                        return cfg;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let file = File::create(&path)?;
            serde_json::to_writer_pretty(file, self)?;
            println!("✅ Config saved to {}", path.display());
        }
        Ok(())
    }
}
