use clap::Parser;
use std::time::Duration;
use update_informer::{registry::Crates, Check, UpdateInformer};
use ureq::Error;
use ureq::{Agent, AgentBuilder};
use colored::*;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    json: bool,
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

    let agent: Agent = AgentBuilder::new().build();
    let body: String = agent.get(url).call()?.into_string()?;
    println!("{}", body);
    Ok(())
}