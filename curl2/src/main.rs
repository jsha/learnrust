use async_std::task;
use http_types;
use std::env;
use std::error;
use std::fmt;
use std::io::{self, Write};
use surf;

#[derive(Debug)]
struct Oops(String);

impl From<io::Error> for Oops {
    fn from(e: io::Error) -> Oops {
        Oops(e.to_string())
    }
}

impl From<http_types::Error> for Oops {
    fn from(e: http_types::Error) -> Oops {
        Oops(e.to_string())
    }
}

impl error::Error for Oops {}
impl fmt::Display for Oops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    for url in env::args().skip(1) {
        task::block_on(async {
            let mut res = surf::get(url).await?;
            io::stdout().write_all(&res.body_bytes().await?)?;
            Ok::<(), Oops>(())
        })?;
    }
    Ok(())
}
