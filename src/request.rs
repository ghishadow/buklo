use std::error;
use std::fmt;
use std::io;

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

impl From<ureq::http::Error> for Error {
    fn from(source: ureq::http::Error) -> Self {
        Error { source: source.into() }
    }
}

pub async fn request(
    method: &str,
    url: &str,
    data: &[u8],
    print_headers: bool,
) -> Result<(), Error> {
    let method = method
        .parse::<ureq::http::Method>()
        .map_err(|err| StringError(format!("invalid HTTP method {method:?}: {err}")))?;
    let agent = ureq::Agent::new_with_defaults();
    let response = if method == ureq::http::Method::GET && data.is_empty() {
        let request = ureq::http::Request::builder()
            .method(method)
            .uri(url)
            .body(())?;
        agent.run(request)?
    } else {
        let request = ureq::http::Request::builder()
            .method(method)
            .uri(url)
            .body(data)?;
        agent.run(request)?
    };
    if print_headers {
        println!(
            "{:?} {} {}",
            response.version(),
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or_default()
        );
        for (name, value) in response.headers() {
            println!("{}: {}", name, value.to_str().unwrap_or_default());
        }
        println!();
    }
    let mut reader = response.into_body().into_reader();
    io::copy(&mut reader, &mut io::stdout())?;
    Ok(())
}
