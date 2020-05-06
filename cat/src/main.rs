use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

fn main() {
    let mut exit_status = 0;
    if env::args().len() == 1 {
        match copy_file_to_stdout("-", io::stdin()) {
            Ok(()) => {}
            Err(()) => exit_status = 1,
        }
    } else {
        for arg in env::args().skip(1) {
            let result = if arg == "-" {
                copy_file_to_stdout("-", io::stdin())
            } else {
                copy_to_stdout(&arg)
            };
            match result {
                Ok(()) => continue,
                Err(()) => exit_status = 1,
            }
        }
    }
    process::exit(exit_status);
}

fn copy_to_stdout(filename: &str) -> Result<(), ()> {
    match File::open(filename) {
        Ok(file) => copy_file_to_stdout(filename, &file),
        Err(s) => {
            let _ = writeln!(io::stderr(), "{}: {}", filename, s);
            Err(())
        }
    }
}

fn copy_file_to_stdout<T: io::Read>(filename: &str, file: T) -> Result<(), ()> {
    let mut reader = io::BufReader::new(file);
    let mut buffer = [0; 10];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                let _ = io::stdout().write(&buffer[..n]);
            }
            Err(s) => {
                let _ = writeln!(io::stderr(), "{}: {}", filename, s);
                return Err(());
            }
        }
    }
}
