use anyhow::{anyhow, Result};
use inquire::Select;
use std::fmt;

use crate::session::SessionInfo;

pub fn highlight_keyword(text: &str, query: &str) -> String {
    if query.trim().is_empty() {
        return text.to_string();
    }

    let text_lower = text.to_lowercase();
    let query_lower = query.to_lowercase();

    let mut result = String::new();
    let mut last_end = 0;

    for (start, _) in text_lower.match_indices(&query_lower) {
        if start < last_end {
            continue;
        }
        result.push_str(&text[last_end..start]);
        let end = start + query_lower.len();
        // Bold yellow ANSI highlight
        result.push_str("\x1b[1;33m");
        result.push_str(&text[start..end]);
        result.push_str("\x1b[0m");
        last_end = end;
    }

    result.push_str(&text[last_end..]);
    result
}

pub struct DisplaySession<'a> {
    pub session: &'a SessionInfo,
    pub query: Option<String>,
}

impl<'a> fmt::Display for DisplaySession<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short_id = if self.session.id.len() >= 8 {
            &self.session.id[..8]
        } else {
            &self.session.id
        };

        let max_len = 55;
        let char_count = self.session.title.chars().count();
        let short_title: String = self.session.title.chars().take(max_len).collect();
        let raw_title = if char_count > max_len {
            format!("{}...", short_title)
        } else {
            short_title
        };

        let (display_time, display_id, display_title) = if let Some(ref q) = self.query {
            (
                highlight_keyword(&self.session.formatted_time(), q),
                highlight_keyword(short_id, q),
                highlight_keyword(&raw_title, q),
            )
        } else {
            (self.session.formatted_time(), short_id.to_string(), raw_title)
        };

        write!(
            f,
            "[{}] ({}) {}",
            display_time,
            display_id,
            display_title
        )
    }
}

pub fn select_session(sessions: &[SessionInfo], query: Option<&str>) -> Result<SessionInfo> {
    if sessions.is_empty() {
        return Err(anyhow!("No matching session history found."));
    }

    let items: Vec<DisplaySession> = sessions
        .iter()
        .map(|s| DisplaySession {
            session: s,
            query: query.map(|q| q.to_string()),
        })
        .collect();

    let ans = Select::new("Select a conversation session to resume (type keyword to filter):", items)
        .with_page_size(10)
        .prompt()?;

    Ok(ans.session.clone())
}
