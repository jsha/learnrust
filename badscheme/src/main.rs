use reqwest::{blocking, Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("hxxps://example.com").unwrap();
    let mut resp = blocking::get(url)?;
    resp.copy_to(&mut std::io::stdout())?;
    Ok(())
}
