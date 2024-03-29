use owo_colors::OwoColorize;
use std::time::Duration;
use update_informer::{registry::Crates, Check};

// TODO implement github release check
pub async fn check_version() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let interval = Duration::from_secs(60 * 60 * 24);
    let informer = update_informer::new(Crates, pkg_name, current_version).interval(interval);
    if let Ok(Some(version)) = informer.check_version() {
        let msg = format!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name.italic().cyan(),
            current_version = current_version,
            new_version = version.to_string().green()
        );

        println!("\n{msg}");
    }
}
