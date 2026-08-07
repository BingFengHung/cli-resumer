use crate::session::SessionInfo;
use crate::ui::highlight_keyword;

pub fn print_session_info(session: &SessionInfo, query: Option<&str>) {
    let ws = session
        .workspace_path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let q = query.unwrap_or("");

    println!("┌────────────────────────────────────────────────────────────────────────┐");
    println!("│ 🔍 Session Information Card                                            │");
    println!("├────────────────────────────────────────────────────────────────────────┤");
    println!("│ Session ID:   {}", highlight_keyword(&session.id, q));
    println!("│ Provider:     {}", session.provider);
    println!("│ Time:         {} ({})", highlight_keyword(&session.formatted_time(), q), session.relative_time());
    println!("│ Workspace:    {}", ws);
    println!("│ Turn Count:   {} user prompt(s)", session.prompt_count);
    println!("├────────────────────────────────────────────────────────────────────────┤");
    println!("│ Prompt Previews:                                                       │");
    if session.prompt_previews.is_empty() {
        println!("│   (No user prompts found)");
    } else {
        for (i, p) in session.prompt_previews.iter().enumerate() {
            let short: String = p.chars().take(60).collect();
            println!("│   {}. {}", i + 1, highlight_keyword(&short, q));
        }
    }
    println!("└────────────────────────────────────────────────────────────────────────┘");
}
