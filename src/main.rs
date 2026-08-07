use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::env;

mod alias;
mod clean;
mod config;
mod info;
mod providers;
mod session;
mod ui;
mod updater;

use config::Config;
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
    #[arg(short, long, value_enum)]
    target: Option<TargetCli>,

    /// Show interactive selection menu instead of automatically auto-resuming the latest
    #[arg(short, long)]
    select: bool,

    /// Filter session history by a search keyword (matches prompt text, title, or ID)
    #[arg(short = 'q', long)]
    query: Option<String>,

    /// Show detailed inspection card for session
    #[arg(short = 'i', long)]
    info: bool,

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
    /// Install convenient shell aliases (agyr, agys, cpr, cps)
    Alias,
    /// Inspect detailed information card for a session
    Info,
    /// Scan and safely clean empty/untitled sessions
    Clean {
        /// Automatically confirm deletion without prompting
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// Manage and edit configuration file (~/.cli-resumer/config.json)
    Config {
        /// Set default target CLI (agy, copilot, auto)
        #[arg(short, long)]
        target: Option<String>,

        /// Set default select mode (true / false)
        #[arg(short, long)]
        select: Option<bool>,

        /// Open config.json in system text editor
        #[arg(short, long)]
        edit: bool,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut cfg = Config::load();

    if args.update || matches!(args.command, Some(Commands::Update)) {
        return updater::check_and_update();
    }

    if matches!(args.command, Some(Commands::Alias)) {
        return alias::install_aliases();
    }

    if let Some(Commands::Clean { yes }) = args.command {
        return clean::clean_empty_sessions(yes);
    }

    if let Some(Commands::Config { target, select, edit }) = args.command {
        if edit {
            return Config::open_in_editor();
        }

        if target.is_none() && select.is_none() {
            return cfg.interactive_edit();
        }

        if let Some(t) = target {
            cfg.default_target = t;
        }
        if let Some(s) = select {
            cfg.default_select = s;
        }
        cfg.save()?;
        println!("✅ Configuration updated!");
        return Ok(());
    }

    let target_mode = args.target.unwrap_or_else(|| match cfg.default_target.as_str() {
        "copilot" => TargetCli::Copilot,
        "auto" => TargetCli::Auto,
        _ => TargetCli::Agy,
    });

    let current_dir = if args.all_workspaces {
        None
    } else {
        env::current_dir().ok()
    };

    let mut sessions: Vec<SessionInfo> = Vec::new();

    match target_mode {
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

    if let Some(explicit_id) = args.id {
        if let Some(found) = sessions.iter().find(|s| s.id == explicit_id) {
            if args.info || matches!(args.command, Some(Commands::Info)) {
                info::print_session_info(found, args.query.as_deref());
                return Ok(());
            }
            match found.provider {
                ProviderType::Agy => return AgyProvider::launch_resume(&found.id),
                ProviderType::Copilot => return CopilotProvider::launch_resume(&found.id),
            }
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
        match target_mode {
            TargetCli::Copilot => return CopilotProvider::launch_resume(""),
            _ => return AgyProvider::launch_resume(""),
        }
    }

    let force_select = args.select || cfg.default_select;
    let selected_session = if force_select || (args.query.is_some() && sessions.len() > 1) {
        ui::select_session(&sessions, args.query.as_deref())?
    } else {
        sessions
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No session available."))?
    };

    if args.info || matches!(args.command, Some(Commands::Info)) {
        info::print_session_info(&selected_session, args.query.as_deref());
        return Ok(());
    }

    println!(
        "Resuming session [{}] ({}) created at {} ({}) ...",
        selected_session.id,
        selected_session.title,
        selected_session.formatted_time(),
        selected_session.relative_time()
    );

    match selected_session.provider {
        ProviderType::Agy => AgyProvider::launch_resume(&selected_session.id),
        ProviderType::Copilot => CopilotProvider::launch_resume(&selected_session.id),
    }
}
