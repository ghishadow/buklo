//! Buklo is a simple CLI HTTP client
//! Usage:
//! ``` bash
//! $ buklo  --url [url] --method [method]
//! ```

mod version;

use crate::version::check_version;
use clap::Parser;
use colored::*;

use std::collections::HashMap;

struct Bulko {
    url: String,
}

impl Bulko {
    fn new(url: String) -> Bulko {
        Bulko { url }
    }
    #[tokio::main]
    async fn get(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.url.clone();
        let res = reqwest::get(&url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", res);
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    method: String,
    #[clap(short, long)]
    url: String,
}

fn main() {
    check_version();
    let args = Args::parse();
    let url = &args.url;
    let method = args.method;
    let bulko = Bulko::new(url.clone());
    match method.as_str() {
        "GET" => {
            bulko.get();
        }
        // "POST" => {
        //     let res = agent.post(url).call()?.into_string()?;
        //     println!("{}", res.yellow());
        // }
        _ => {
            println!("{}", "Method not found".red());
        }
    }
}
