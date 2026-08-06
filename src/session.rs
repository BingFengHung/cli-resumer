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

pub fn clean_prompt_text(raw: &str) -> String {
    let mut in_tag = false;
    let mut cleaned = String::new();

    for c in raw.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
            cleaned.push(' ');
        } else if !in_tag {
            cleaned.push(c);
        }
    }

    let single_line = cleaned
        .replace('\r', " ")
        .replace('\n', " ")
        .replace('\t', " ");

    let words: Vec<&str> = single_line.split_whitespace().collect();
    let result = words.join(" ");

    if result.is_empty() {
        "Untitled Session".to_string()
    } else {
        result
    }
}
