use anyhow::{anyhow, Result};
use inquire::Select;
use std::fmt;

use crate::session::SessionInfo;

struct DisplaySession<'a>(&'a SessionInfo);

impl<'a> fmt::Display for DisplaySession<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] [{}] {} - {}",
            self.0.provider,
            self.0.formatted_time(),
            self.0.id,
            self.0.title
        )
    }
}

pub fn select_session(sessions: &[SessionInfo]) -> Result<SessionInfo> {
    if sessions.is_empty() {
        return Err(anyhow!("No matching session history found."));
    }

    let items: Vec<DisplaySession> = sessions.iter().map(DisplaySession).collect();

    let ans = Select::new("Select a conversation session to resume:", items)
        .with_page_size(10)
        .prompt()?;

    Ok(ans.0.clone())
}
