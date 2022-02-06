//! Buklo is a simple CLI HTTP client
//! Usage:
//! ``` bash
//! $ buklo  --url [url] --method [method] 
//! ```


use clap::Parser;
use colored::*;
use std::time::Duration;
use update_informer::{registry::Crates, Check, UpdateInformer};
use ureq::Error;
use ureq::{Agent, AgentBuilder};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    method: String,
    #[clap(short, long)]
    url: String,
}

fn check_version() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let current_version = env!("CARGO_PKG_VERSION");
    let interval = Duration::from_secs(60 * 60 * 24);
    let informer = UpdateInformer::new(Crates, pkg_name, current_version, interval);
    if let Ok(Some(version)) = informer.check_version() {
        let msg = format!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name.italic().cyan(),
            current_version = current_version,
            new_version = version.to_string().green()
        );

        println!("\n{msg}", msg = msg);
    }
}

fn main() -> Result<(), ureq::Error> {
    check_version();
    let args = Args::parse();
    let url = &args.url;
    let method = args.method;

    let agent: Agent = AgentBuilder::new().build();
    match method.as_str() {
        "GET" => {
            let res = agent.get(url).call()?.into_string()?;
            println!("{}", res.yellow());
        }
        _ => {
            println!("{}", "Method not found".red());
        }
    }
    Ok(())
}

