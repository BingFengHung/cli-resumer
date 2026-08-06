use anyhow::Result;

pub fn check_and_update() -> Result<()> {
    println!("Checking GitHub Releases for updates (Repo: BingFengHung/cli-resumer)...");
    println!("Current version: v{}", env!("CARGO_PKG_VERSION"));

    let target = self_update::get_target();
    println!("Detected system target: {}", target);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("BingFengHung")
        .repo_name("cli-resumer")
        .bin_name("cli-resumer")
        .target(&target)
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    if status.updated() {
        println!("✨ Successfully updated cli-resumer to version v{}!", status.version());
    } else {
        println!("✅ cli-resumer is already up to date (v{}).", env!("CARGO_PKG_VERSION"));
    }

    Ok(())
}
