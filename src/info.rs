use crate::session::SessionInfo;

pub fn print_session_info(session: &SessionInfo) {
    let ws = session
        .workspace_path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    println!("┌────────────────────────────────────────────────────────────────────────┐");
    println!("│ 🔍 Session Information Card                                            │");
    println!("├────────────────────────────────────────────────────────────────────────┤");
    println!("│ Session ID:   {}", session.id);
    println!("│ Provider:     {}", session.provider);
    println!("│ Time:         {} ({})", session.formatted_time(), session.relative_time());
    println!("│ Workspace:    {}", ws);
    println!("│ Turn Count:   {} user prompt(s)", session.prompt_count);
    println!("├────────────────────────────────────────────────────────────────────────┤");
    println!("│ Prompt Previews:                                                       │");
    if session.prompt_previews.is_empty() {
        println!("│   (No user prompts found)");
    } else {
        for (i, p) in session.prompt_previews.iter().enumerate() {
            let short: String = p.chars().take(60).collect();
            println!("│   {}. {}", i + 1, short);
        }
    }
    println!("└────────────────────────────────────────────────────────────────────────┘");
}
