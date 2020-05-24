use reqwest::blocking;
use reqwest::Url;
use std::env;
use std::error;
use std::fmt;
use std::io;
use std::io::Write;
use std::process;

#[derive(Debug)]
struct CurlError<T: error::Error>(String, T);

impl<T: error::Error> error::Error for CurlError<T> {}

impl<T: error::Error> fmt::Display for CurlError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "fetching {}: {}", self.0, self.1)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let stdout = io::stdout();
    let urls = env::args().skip(1).collect::<Vec<String>>();
    let result = fetch(&urls, &mut stdout.lock());
    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn fetch(urls: &[String], w: &mut dyn Write) -> Result<(), Box<dyn error::Error>> {
    for url in urls {
        let parsed_url = match Url::parse(&url) {
            Ok(u) => u,
            Err(e) => return Err(Box::new(CurlError(url.to_string(), e))),
        };
        println!("parsed url {}", parsed_url);
        let mut resp = blocking::get(parsed_url)?;
        resp.copy_to(w)?;
    }
    Ok(())
}
