use std::env;
use std::fs::File;
use std::io;
use std::io::stderr;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;

fn main() {
    for arg in env::args().skip(1) {
        match copy_to_stdout(&arg) {
            Ok(()) => continue,
            Err(()) => std::process::exit(1),
        }
    }
}

fn copy_to_stdout(filename: &str) -> Result<(), ()> {
    match File::open(filename) {
        Ok(file) => {
            for line in io::BufReader::new(file).lines() {
                if let Ok(line) = line {
                    let _ = writeln!(stdout(), "{}", line);
                }
            }
            Ok(())
        }
        Err(s) => {
            let _ = writeln!(stderr(), "{}: {}\n", filename, s);
            Err(())
        }
    }
}
