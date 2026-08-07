use anyhow::{anyhow, Result};
use dirs::home_dir;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::providers::{AgyProvider, CopilotProvider};
use crate::session::{clean_prompt_text, SessionInfo};

pub fn export_session(
    target_session: Option<&SessionInfo>,
    output_filename: Option<&str>,
) -> Result<PathBuf> {
    let session = target_session.ok_or_else(|| anyhow!("No session found to export."))?;

    let filename = output_filename.unwrap_or("AI_SESSION_NOTES.md");
    let output_path = std::env::current_dir()?.join(filename);

    let home = home_dir().unwrap_or_default();

    let mut markdown = String::new();
    markdown.push_str("# 📝 AI Conversation Transcript Notes\n\n");
    markdown.push_str(&format!("- **Session ID**: `{}`\n", session.id));
    markdown.push_str(&format!("- **Provider**: `{}`\n", session.provider));
    markdown.push_str(&format!("- **Timestamp**: `{}` ({})\n", session.formatted_time(), session.relative_time()));
    if let Some(ref ws) = session.workspace_path {
        markdown.push_str(&format!("- **Workspace**: `{}`\n", ws.display()));
    }
    markdown.push_str("\n---\n\n");

    let mut dialog_entries = Vec::new();

    let brain_dir = home.join(".gemini").join("antigravity-cli").join("brain").join(&session.id);
    let transcript_full = brain_dir.join(".system_generated").join("logs").join("transcript_full.jsonl");
    let transcript_normal = brain_dir.join(".system_generated").join("logs").join("transcript.jsonl");

    let target_transcript = if transcript_full.exists() {
        Some(transcript_full)
    } else if transcript_normal.exists() {
        Some(transcript_normal)
    } else {
        None
    };

    if let Some(t_path) = target_transcript {
        if let Ok(file) = File::open(t_path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                if let Ok(v) = serde_json::from_str::<Value>(&line) {
                    let type_str = v.get("type").and_then(|t| t.as_str()).unwrap_or_default();
                    let content = v.get("content").and_then(|c| c.as_str()).unwrap_or_default();

                    if type_str == "USER_INPUT" && !content.trim().is_empty() {
                        let cleaned = clean_prompt_text(content);
                        if !cleaned.is_empty() && cleaned != "Untitled Session" {
                            dialog_entries.push(("👤 User", cleaned));
                        }
                    } else if (type_str == "PLANNER_RESPONSE" || type_str == "MODEL_RESPONSE") && !content.trim().is_empty() {
                        dialog_entries.push(("🤖 Assistant", content.trim().to_string()));
                    }
                }
            }
        }
    }

    if dialog_entries.is_empty() {
        markdown.push_str(&format!("### 👤 User Prompt\n\n{}\n\n", session.title));
    } else {
        for (speaker, text) in dialog_entries {
            markdown.push_str(&format!("### {}\n\n{}\n\n---\n\n", speaker, text));
        }
    }

    let mut out_file = File::create(&output_path)?;
    out_file.write_all(markdown.as_bytes())?;

    println!("✅ Session exported successfully to: {}", output_path.display());
    Ok(output_path)
}

pub fn install_export_skill() -> Result<()> {
    let home = home_dir().unwrap_or_default();
    let skill_dir = home
        .join(".gemini")
        .join("antigravity-cli")
        .join("builtin")
        .join("skills")
        .join("export");

    fs::create_dir_all(&skill_dir)?;
    let skill_file = skill_dir.join("SKILL.md");

    let skill_content = r#"---
name: export
description: Export the current conversation transcript into AI_SESSION_NOTES.md
---

# AGY CLI Export Slash Command

When the user types `/export` or requests to export conversation notes:
Execute `cli-resumer export` to export the current session transcript into `AI_SESSION_NOTES.md` in the current workspace.
"#;

    let mut file = File::create(&skill_file)?;
    file.write_all(skill_content.as_bytes())?;
    println!("✅ Registered AGY CLI `/export` Slash Command skill at:");
    println!("   {}", skill_file.display());

    Ok(())
}
