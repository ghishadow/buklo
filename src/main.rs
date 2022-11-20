use argh::FromArgs;
mod version;
mod request;

use crate::request::request;

use crate::version::check_version;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[derive(FromArgs)]
/// Test
struct Args {
    /// method Name
    #[argh(option, short = 'm')]
    method: String,
    /// url
    #[argh(option, short = 'u')]
    url: String,
    /// headers
    #[argh(option, default = "true")]
    headers: bool,
    /// body
    #[argh(option, short = 'b')]
    body: Option<String>,
}

#[tokio::main]
async fn main() {
    /*
        env_logger::init();
        info!("Starting up Buklo");
        info!("Checking for updates");
    */
    check_version().await;
    match start().await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

async fn start() -> Result<(), request::Error> {
    let args: Args = argh::from_env();
    let builder = ureq::builder();

    let mut print_headers: bool = false;
    let method: String = args.method;
    let body: Vec<u8> = args.body.as_deref().map_or(Vec::new(), |s| s.as_bytes().to_vec());
    let url = args.url;

    if args.headers {
        print_headers = true;
    }

    let agent = builder.build();
    request(&agent, &method, &url, &body, print_headers).await?;

    Ok(())
}
