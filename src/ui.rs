use anyhow::{anyhow, Result};
use inquire::Select;
use std::fmt;

use crate::session::SessionInfo;

pub struct DisplaySession<'a>(pub &'a SessionInfo);

impl<'a> fmt::Display for DisplaySession<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short_id = if self.0.id.len() >= 8 {
            &self.0.id[..8]
        } else {
            &self.0.id
        };

        let max_len = 55;
        let char_count = self.0.title.chars().count();
        let short_title: String = self.0.title.chars().take(max_len).collect();
        let display_title = if char_count > max_len {
            format!("{}...", short_title)
        } else {
            short_title
        };

        write!(
            f,
            "[{}] ({}) {}",
            self.0.formatted_time(),
            short_id,
            display_title
        )
    }
}

pub fn select_session(sessions: &[SessionInfo]) -> Result<SessionInfo> {
    if sessions.is_empty() {
        return Err(anyhow!("No matching session history found."));
    }

    let items: Vec<DisplaySession> = sessions.iter().map(DisplaySession).collect();

    let ans = Select::new("Select a conversation session to resume (type keyword to filter):", items)
        .with_page_size(10)
        .prompt()?;

    Ok(ans.0.clone())
}
