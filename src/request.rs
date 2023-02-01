use std::io;
use std::error;
use std::fmt;
use spinoff::{Spinner, spinners, Color, Streams};


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
pub struct Error {
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
        Error { source: source.into() }
    }
}

impl From<ureq::Error> for Error {
    fn from(source: ureq::Error) -> Self {
        Error { source: source.into() }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Error { source: source.into() }
    }
}


pub async fn request(
    agent: &ureq::Agent,
    method: &str,
    url: &str,
    data: &[u8],
    print_headers: bool,
) -> Result<(), Error> {
    let spinner = Spinner::new_with_stream(spinners::Line, "Requesting...", Color::Yellow, Streams::Stderr);
    let req = agent.request(method, url);
    spinner.stop_and_persist("📜", "Request done.");
    let response =
        if method == "GET" && data.is_empty() { req.call()? } else { req.send_bytes(data)? };
    if print_headers {
        println!("{} {} {}", response.http_version(), response.status(), response.status_text());
        response.headers_names().into_iter().for_each(|h| {
            println!("{}: {}", h, response.header(&h).unwrap_or_default());
        });
        println!();
    }
    let mut reader = response.into_reader();
    io::copy(&mut reader, &mut io::stdout())?;
    Ok(())
}