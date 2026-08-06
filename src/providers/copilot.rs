use anyhow::Result;
use chrono::Utc;
use dirs::home_dir;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::session::{ProviderType, SessionInfo};

pub struct CopilotProvider;

impl CopilotProvider {
    pub fn get_sessions(current_dir: Option<&Path>) -> Result<Vec<SessionInfo>> {
        let home = match home_dir() {
            Some(h) => h,
            None => return Ok(Vec::new()),
        };

        let possible_dirs = vec![
            home.join(".copilot-cli").join("history"),
            home.join(".copilot-cli").join("sessions"),
            home.join(".config").join("github-copilot").join("sessions"),
        ];

        let mut sessions = Vec::new();

        for dir in possible_dirs {
            if !dir.exists() {
                continue;
            }

            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let session_id = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let mut title = "Copilot Session".to_string();
                    let mut timestamp = Utc::now();
                    let mut workspace_path: Option<PathBuf> = None;

                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            timestamp = modified.into();
                        }
                    }

                    if let Ok(file) = File::open(&path) {
                        let reader = BufReader::new(file);
                        if let Ok(v) = serde_json::from_reader::<_, Value>(reader) {
                            if let Some(cwd) = v.get("cwd").and_then(|c| c.as_str()) {
                                workspace_path = Some(PathBuf::from(cwd));
                            }
                            if let Some(prompt) = v.get("prompt").or(v.get("title")).and_then(|t| t.as_str()) {
                                title = prompt.trim().chars().take(60).collect();
                            }
                        }
                    }

                    let matches_cwd = if let (Some(cwd), Some(ref ws)) = (current_dir, &workspace_path) {
                        ws.canonicalize().ok() == cwd.canonicalize().ok() || ws == cwd
                    } else {
                        true
                    };

                    if current_dir.is_none() || matches_cwd {
                        sessions.push(SessionInfo {
                            id: session_id,
                            title,
                            timestamp,
                            workspace_path,
                            provider: ProviderType::Copilot,
                        });
                    }
                }
            }
        }

        sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(sessions)
    }

    pub fn launch_resume(session_id: &str) -> Result<()> {
        println!("Launching GitHub Copilot CLI with session [{}]...", session_id);
        
        #[cfg(target_os = "windows")]
        let mut cmd = std::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        cmd.args(["/C", "copilot", "resume", session_id]);

        #[cfg(not(target_os = "windows"))]
        let mut cmd = std::process::Command::new("copilot");
        #[cfg(not(target_os = "windows"))]
        cmd.args(["resume", session_id]);

        let status = cmd.status()?;
        if !status.success() {
            println!("Fallback: Attempting to launch gh copilot...");
            #[cfg(target_os = "windows")]
            let _ = std::process::Command::new("cmd").args(["/C", "gh", "copilot", "suggest"]).status();
            #[cfg(not(target_os = "windows"))]
            let _ = std::process::Command::new("gh").args(["copilot", "suggest"]).status();
        }

        Ok(())
    }
}
