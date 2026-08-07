use anyhow::Result;
use dirs::home_dir;
use inquire::{Confirm, CustomType, Select};
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
        home_dir().map(|d| d.join(".cli-resumer").join("config.json"))
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
        let cfg = Self::default();
        let _ = cfg.save();
        cfg
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let file = File::create(&path)?;
            serde_json::to_writer_pretty(file, self)?;
        }
        Ok(())
    }

    pub fn interactive_edit(&mut self) -> Result<()> {
        println!("⚙️ Interactive Config Setup for cli-resumer");

        let targets = vec!["agy", "copilot", "auto"];
        let target_idx = targets.iter().position(|&t| t == self.default_target).unwrap_or(0);
        
        let selected_target = Select::new("Default Target CLI Tool:", targets)
            .with_starting_cursor(target_idx)
            .prompt()?;
        self.default_target = selected_target.to_string();

        let default_select = Confirm::new("Default to interactive session selection menu on launch?")
            .with_default(self.default_select)
            .prompt()?;
        self.default_select = default_select;

        let page_size = CustomType::<usize>::new("Interactive Menu Page Size:")
            .with_default(self.page_size)
            .prompt()?;
        self.page_size = page_size;

        self.save()?;
        if let Some(path) = Self::path() {
            println!("✅ Configuration updated successfully: {}", path.display());
        }
        Ok(())
    }

    pub fn open_in_editor() -> Result<()> {
        let path = Self::path().ok_or_else(|| anyhow::anyhow!("Config path not found"))?;
        if !path.exists() {
            Self::default().save()?;
        }

        println!("Opening {} in default editor...", path.display());

        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", "", path.to_str().unwrap_or_default()])
                .status();
        }

        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&path).status();
        }

        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(&path).status();
        }

        Ok(())
    }
}
