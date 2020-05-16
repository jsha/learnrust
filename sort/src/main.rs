use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

fn main() -> io::Result<()> {
    let mut lines: Vec<Vec<u8>> = vec![];
    for file in env::args().skip(1) {
        let r = io::BufReader::new(File::open(file)?);
        for line in r.split(b'\n') {
            lines.push(line?);
        }
    }
    lines.sort();
    for line in lines {
        io::stdout().write_all(&line)?;
        io::stdout().write_all(&[b'\n'])?;
    }
    Ok(())
}
