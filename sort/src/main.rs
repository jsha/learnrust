#![feature(is_sorted)]
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

type Line = Vec<u8>;
type LineOrdering = dyn Fn(&Line, &Line) -> std::cmp::Ordering;

fn main() -> io::Result<()> {
    let conf = match process_args() {
        Ok(conf) => conf,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let mut lines: Vec<Vec<u8>> = vec![];
    for file in &(conf.non_flags) {
        let r = io::BufReader::new(File::open(file)?);
        for line in r.split(b'\n') {
            lines.push(line?);
        }
    }
    let comparator = make_comparator(&conf);

    if conf.check_order {
        if lines.is_sorted_by(|a, b| Some(comparator(a, b))) {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    lines.sort_by(comparator);
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

fn make_comparator(conf: &Config) -> Box<LineOrdering> {
    if conf.reverse {
        Box::new(&|a: &Line, b: &Line| a.cmp(b))
    } else {
        Box::new(&|a: &Line, b: &Line| b.cmp(a))
    }
}

#[derive(Default)]
struct Config {
    unique: bool,
    reverse: bool,
    check_order: bool,
    non_flags: Vec<String>,
}

fn process_args() -> Result<Config, String> {
    let mut conf = Config::default();

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-u" => conf.unique = true,
            "-r" => conf.reverse = true,
            "-c" => conf.check_order = true,
            "-h" => {
                return Err(format!(
                    "Usage: {} [-u] [-r] [file1 [file2 ...]]",
                    env::args().next().unwrap_or_else(|| "sort".to_string())
                ))
            }
            _ => conf.non_flags.push(arg),
        }
    }
    Ok(conf)
}
