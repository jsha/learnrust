use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

fn main() -> io::Result<()> {
    let conf = match process_args() {
        Ok(conf) => conf,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let mut lines: Vec<Vec<u8>> = vec![];
    for file in conf.non_flags {
        let r = io::BufReader::new(File::open(file)?);
        for line in r.split(b'\n') {
            lines.push(line?);
        }
    }
    lines.sort();
    let mut last: Option<&Vec<u8>> = None;
    for line in lines.iter() {
        if conf.unique && last.is_some() && line == last.unwrap() {
            continue;
        }
        io::stdout().write_all(&line)?;
        io::stdout().write_all(&[b'\n'])?;
        last = Some(line);
    }
    Ok(())
}

struct Config {
    unique: bool,
    non_flags: Vec<String>,
}

impl Config {
    fn new() -> Config {
        Config {
            unique: false,
            non_flags: Vec::new(),
        }
    }
}

fn process_args() -> Result<Config, String> {
    let mut conf = Config::new();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-u" => conf.unique = true,
            "-h" => {
                return Err(format!(
                    "Usage: {} [-u] [file1 [file2 ...]]",
                    env::args().next().unwrap_or_else(|| "sort".to_string())
                ))
            }
            _ => conf.non_flags.push(arg),
        }
    }
    Ok(conf)
}
