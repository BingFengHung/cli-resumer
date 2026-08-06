use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use std::env;

mod providers;
mod session;
mod ui;

use providers::{AgyProvider, CopilotProvider};
use session::{ProviderType, SessionInfo};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum TargetCli {
    Agy,
    Copilot,
    Auto,
}

#[derive(Parser, Debug)]
#[command(
    name = "cli-resumer",
    author = "Developer",
    version = "0.1.0",
    about = "Automatically resume your last AI CLI conversation or pick from session history."
)]
struct Args {
    /// Target CLI tool to resume (agy, copilot, auto)
    #[arg(short, long, value_enum, default_value = "agy")]
    target: TargetCli,

    /// Show interactive selection menu instead of automatically auto-resuming the latest
    #[arg(short, long)]
    select: bool,

    /// Include sessions from all workspaces (ignore current working directory filtering)
    #[arg(short = 'a', long)]
    all_workspaces: bool,

    /// Directly resume a specific Session ID
    #[arg(long)]
    id: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let current_dir = if args.all_workspaces {
        None
    } else {
        env::current_dir().ok()
    };

    if let Some(explicit_id) = args.id {
        match args.target {
            TargetCli::Copilot => return CopilotProvider::launch_resume(&explicit_id),
            _ => return AgyProvider::launch_resume(&explicit_id),
        }
    }

    let mut sessions: Vec<SessionInfo> = Vec::new();

    match args.target {
        TargetCli::Agy => {
            sessions.extend(AgyProvider::get_sessions(current_dir.as_deref())?);
        }
        TargetCli::Copilot => {
            sessions.extend(CopilotProvider::get_sessions(current_dir.as_deref())?);
        }
        TargetCli::Auto => {
            sessions.extend(AgyProvider::get_sessions(current_dir.as_deref())?);
            sessions.extend(CopilotProvider::get_sessions(current_dir.as_deref())?);
            sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        }
    }

    if sessions.is_empty() {
        println!("No previous session history found for current directory.");
        println!("Launching standard CLI session...");
        match args.target {
            TargetCli::Copilot => return CopilotProvider::launch_resume(""),
            _ => return AgyProvider::launch_resume(""),
        }
    }

    let selected_session = if args.select {
        ui::select_session(&sessions)?
    } else {
        sessions
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No session available."))?
    };

    println!(
        "Resuming session [{}] ({}) created at {}...",
        selected_session.id,
        selected_session.title,
        selected_session.formatted_time()
    );

    match selected_session.provider {
        ProviderType::Agy => AgyProvider::launch_resume(&selected_session.id),
        ProviderType::Copilot => CopilotProvider::launch_resume(&selected_session.id),
    }
}
