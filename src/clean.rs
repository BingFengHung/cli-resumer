use anyhow::Result;
use inquire::Confirm;
use std::fs;

use crate::providers::{AgyProvider, CopilotProvider};

pub fn clean_empty_sessions(auto_confirm: bool) -> Result<()> {
    println!("Scanning for empty/untitled sessions...");

    let agy_sessions = AgyProvider::get_sessions(None)?;
    let copilot_sessions = CopilotProvider::get_sessions(None)?;

    let mut empty_sessions = Vec::new();

    for s in agy_sessions.into_iter().chain(copilot_sessions.into_iter()) {
        if s.prompt_count == 0 || s.title == "Untitled Session" {
            empty_sessions.push(s);
        }
    }

    if empty_sessions.is_empty() {
        println!("✨ No empty sessions found. All session history is clean!");
        return Ok(());
    }

    println!("Found {} empty/untitled session(s):", empty_sessions.len());
    for s in &empty_sessions {
        println!("  - [{}] ({}) created at {}", s.provider, s.id, s.formatted_time());
    }

    let should_delete = if auto_confirm {
        true
    } else {
        Confirm::new("Are you sure you want to delete these empty session directories?")
            .with_default(false)
            .prompt()?
    };

    if !should_delete {
        println!("Cancelled. No sessions were deleted.");
        return Ok(());
    }

    let mut deleted_count = 0;
    let home = dirs::home_dir().unwrap_or_default();
    let agy_brain = home.join(".gemini").join("antigravity-cli").join("brain");

    for s in empty_sessions {
        let dir_path = agy_brain.join(&s.id);
        if dir_path.exists() {
            if fs::remove_dir_all(&dir_path).is_ok() {
                deleted_count += 1;
            }
        }
    }

    println!("🧹 Cleaned up {} empty session directory(ies).", deleted_count);
    Ok(())
}
