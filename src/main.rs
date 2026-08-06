use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::env;

mod providers;
mod session;
mod ui;
mod updater;

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
    author = "BingFengHung",
    version = env!("CARGO_PKG_VERSION"),
    about = "Automatically resume your last AI CLI conversation or pick from session history."
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Target CLI tool to resume (agy, copilot, auto)
    #[arg(short, long, value_enum, default_value = "agy")]
    target: TargetCli,

    /// Show interactive selection menu instead of automatically auto-resuming the latest
    #[arg(short, long)]
    select: bool,

    /// Filter session history by a search keyword (matches prompt text, title, or ID)
    #[arg(short = 'q', long)]
    query: Option<String>,

    /// Include sessions from all workspaces (ignore current working directory filtering)
    #[arg(short = 'a', long)]
    all_workspaces: bool,

    /// Check and self-update cli-resumer from GitHub Releases
    #[arg(short = 'u', long)]
    update: bool,

    /// Directly resume a specific Session ID
    #[arg(long)]
    id: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check GitHub Releases and update cli-resumer to the latest version
    Update,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.update || matches!(args.command, Some(Commands::Update)) {
        return updater::check_and_update();
    }

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

    if let Some(ref q) = args.query {
        let q_lower = q.to_lowercase();
        sessions.retain(|s| {
            s.title.to_lowercase().contains(&q_lower)
                || s.id.to_lowercase().contains(&q_lower)
                || s.formatted_time().contains(&q_lower)
        });

        if sessions.is_empty() {
            println!("No session history found matching query: '{}'", q);
            return Ok(());
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

    let selected_session = if args.select || (args.query.is_some() && sessions.len() > 1) {
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
