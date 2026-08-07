use anyhow::Result;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;

use crate::export;

pub fn install_aliases() -> Result<()> {
    println!("Installing shell aliases and AGY CLI skills for cli-resumer...");

    let _ = export::install_export_skill();

    #[cfg(target_os = "windows")]
    {
        println!("Detecting Windows PowerShell profile...");
        let ps_profile = home_dir()
            .unwrap_or_default()
            .join("Documents")
            .join("WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1");

        if let Some(parent) = ps_profile.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&ps_profile)?;

        let alias_content = r#"
# cli-resumer Shell Aliases
function agyr { cli-resumer -t agy @args }
function agys { cli-resumer -t agy -s @args }
function cpr  { cli-resumer -t copilot @args }
function cps  { cli-resumer -t copilot -s @args }
"#;
        file.write_all(alias_content.as_bytes())?;
        println!("✅ Added PowerShell aliases (agyr, agys, cpr, cps) to:");
        println!("   {}", ps_profile.display());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home = home_dir().unwrap_or_default();
        let target_rc = if home.join(".zshrc").exists() {
            home.join(".zshrc")
        } else {
            home.join(".bashrc")
        };

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&target_rc)?;

        let alias_content = r#"
# cli-resumer Shell Aliases
alias agyr="cli-resumer -t agy"
alias agys="cli-resumer -t agy -s"
alias cpr="cli-resumer -t copilot"
alias cps="cli-resumer -t copilot -s"
"#;
        file.write_all(alias_content.as_bytes())?;
        println!("✅ Added shell aliases (agyr, agys, cpr, cps) to:");
        println!("   {}", target_rc.display());
    }

    println!("\nRestart your terminal or reload your shell profile to start using the aliases!");
    Ok(())
}
