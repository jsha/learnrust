use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process;

fn main() {
    if env::args().len() == 1 {
        match copy_file_to_stdout("-", io::stdin()) {
            Ok(()) => {}
            Err(()) => process::exit(1),
        }
    }
    for arg in env::args().skip(1) {
        match copy_to_stdout(&arg) {
            Ok(()) => continue,
            Err(()) => process::exit(1),
        }
    }
}

fn copy_to_stdout(filename: &str) -> Result<(), ()> {
    match File::open(filename) {
        Ok(file) => copy_file_to_stdout(filename, &file),
        Err(s) => {
            let _ = writeln!(io::stderr(), "{}: {}\n", filename, s);
            Err(())
        }
    }
}

fn copy_file_to_stdout<T: io::Read>(filename: &str, file: T) -> Result<(), ()> {
    // TODO: If there is a read error, print whatever partial line was read before the error.
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                let _ = writeln!(io::stdout(), "{}", line);
            }
            Err(s) => {
                let _ = writeln!(io::stderr(), "{}: {}\n", filename, s);
                return Err(());
            }
        }
    }
    Ok(())
}
