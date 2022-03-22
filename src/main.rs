use clap::Parser;
use std::error;
use std::fmt;
use std::io;
mod version;
use crate::version::check_version;

#[derive(Debug)]
struct StringError(String);

impl error::Error for StringError {}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for StringError {
    fn from(source: String) -> Self {
        Self(source)
    }
}

#[derive(Debug)]
struct Error {
    source: Box<dyn error::Error>,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl From<StringError> for Error {
    fn from(source: StringError) -> Self {
        Error {
            source: source.into(),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(source: ureq::Error) -> Self {
        Error {
            source: source.into(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error {
            source: source.into(),
        }
    }
}

fn request(
    agent: &ureq::Agent,
    method: &str,
    url: &str,
    data: &[u8],
    print_headers: bool,
) -> Result<(), Error> {
    let req = agent.request(method, url);
    let response = if method == "GET" && data.len() == 0 {
        req.call()?
    } else {
        req.send_bytes(data)?
    };
    if print_headers {
        println!(
            "{} {} {}",
            response.http_version(),
            response.status(),
            response.status_text()
        );
        for h in response.headers_names() {
            println!("{}: {}", h, response.header(&h).unwrap_or_default());
        }
        println!();
    }
    let mut reader = response.into_reader();
    io::copy(&mut reader, &mut io::stdout())?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    method: String,
    #[clap(short, long)]
    url: String,
    #[clap(short, long)]
    headers: bool,
    #[clap(short, long)]
    body: Option<String>,
}

fn main() {
    check_version();
    match start() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn start() -> Result<(), Error> {
    let args = Args::parse();
    let builder = ureq::builder();

    let mut print_headers: bool = false;
    let method: String = args.method;
    let body: Vec<u8> = args
        .body
        .as_deref()
        .map_or(Vec::new(), |s| s.as_bytes().to_vec());
    let url = args.url;

    if args.headers {
        print_headers = true;
    }

    let agent = builder.build();
    request(&agent, &method, &url, &body, print_headers)?;

    Ok(())
}
