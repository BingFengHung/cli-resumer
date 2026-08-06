use anyhow::Result;
use chrono::{TimeZone, Utc};
use dirs::home_dir;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::session::{ProviderType, SessionInfo};

pub struct AgyProvider;

impl AgyProvider {
    pub fn get_sessions(current_dir: Option<&Path>) -> Result<Vec<SessionInfo>> {
        let home = match home_dir() {
            Some(h) => h,
            None => return Ok(Vec::new()),
        };

        let brain_dir = home.join(".gemini").join("antigravity-cli").join("brain");
        if !brain_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();

        let entries = match fs::read_dir(&brain_dir) {
            Ok(e) => e,
            Err(_) => return Ok(Vec::new()),
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let session_id = match path.file_name().and_then(|n| n.to_str()) {
                Some(id) => id.to_string(),
                None => continue,
            };

            let logs_dir = path.join(".system_generated").join("logs");
            let transcript_path = logs_dir.join("transcript.jsonl");

            let mut title = "Untitled Session".to_string();
            let mut timestamp = Utc::now();
            let mut workspace_path: Option<PathBuf> = None;

            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    timestamp = modified.into();
                }
            }

            if transcript_path.exists() {
                if let Ok(file) = File::open(&transcript_path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        if let Ok(v) = serde_json::from_str::<Value>(&line) {
                            if workspace_path.is_none() {
                                if let Some(ws) = v.get("workspace").and_then(|w| w.as_str()) {
                                    workspace_path = Some(PathBuf::from(ws));
                                }
                            }

                            if title == "Untitled Session" {
                                if let Some(type_str) = v.get("type").and_then(|t| t.as_str()) {
                                    if type_str == "USER_INPUT" {
                                        if let Some(content) = v.get("content").and_then(|c| c.as_str()) {
                                            let cleaned = content.trim();
                                            if !cleaned.is_empty() {
                                                title = cleaned.chars().take(60).collect();
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                    provider: ProviderType::Agy,
                });
            }
        }

        sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(sessions)
    }

    pub fn launch_resume(session_id: &str) -> Result<()> {
        let is_latest = session_id.is_empty();
        println!(
            "Launching AGY CLI with session [{}]...",
            if is_latest { "latest" } else { session_id }
        );

        let mut command_args = Vec::new();
        if is_latest {
            command_args.push("--continue".to_string());
        } else {
            command_args.push("--conversation".to_string());
            command_args.push(session_id.to_string());
        }

        #[cfg(target_os = "windows")]
        let mut cmd = std::process::Command::new("cmd");
        #[cfg(target_os = "windows")]
        {
            let mut full_args = vec!["/C".to_string(), "agy".to_string()];
            full_args.extend(command_args);
            cmd.args(&full_args);
        }

        #[cfg(not(target_os = "windows"))]
        let mut cmd = std::process::Command::new("agy");
        #[cfg(not(target_os = "windows"))]
        cmd.args(&command_args);

        let status = cmd.status()?;
        if !status.success() {
            println!("Note: Failed to launch agy with conversation arguments. Falling back to launching standard 'agy'...");
            #[cfg(target_os = "windows")]
            let _ = std::process::Command::new("cmd").args(["/C", "agy"]).status();
            #[cfg(not(target_os = "windows"))]
            let _ = std::process::Command::new("agy").status();
        }

        Ok(())
    }
}
