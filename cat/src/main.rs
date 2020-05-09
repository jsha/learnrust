use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::process;

fn main() {
    let mut output: Box<dyn io::Write> = Box::new(io::stdout());
    let show_line_numbers = env::var("NUMBER").unwrap_or_else(|_| "0".to_string());
    if show_line_numbers == "1" {
        output = Box::new(NumberedOut::new());
    }
    process_args(env::args(), &mut output);
}

// process_args reads each file named in the command line args, and writes
// the contents to output. If there are no args, copy stdin to output.
fn process_args(args: env::Args, output: &mut dyn io::Write) {
    let mut exit_status = 0;
    if args.len() == 1 {
        match copy_file_to("-", &mut io::stdin(), output) {
            Ok(()) => {}
            Err(e) => {
                // flush output before printing an error so that, when
                // run on a terminal, the error shows up in the right place
                // relative to the output above and below it. Otherwise
                // buffering means much of the regular output will get
                // displayed after the error, even if it was output before
                // the error.
                output.flush().unwrap();
                eprintln!("{}", e);
                exit_status = 1
            }
        }
    } else {
        for arg in args.skip(1) {
            let result = if arg == "-" {
                copy_file_to("-", &mut io::stdin(), output)
            } else {
                copy_to(&arg, output)
            };
            match result {
                Ok(()) => continue,
                Err(e) => {
                    output.flush().unwrap();
                    eprintln!("{}", e);
                    // Don't exit immediately on error. Try to read any
                    // remaining files. Mimics GNU cat.
                    exit_status = 1;
                }
            }
        }
    }
    output.flush().expect("failed to flush output");
    process::exit(exit_status);
}

// NumberedOut implements Write by writing output to stdout, prefixed by
// line numbers (starting with 1). Line numbers are only printed when there
// are more bytes to print after them, so a file that ends in a newline
// won't have an additional number printed after the last line.
struct NumberedOut {
    n: i64,
    beginning_line: bool,
    output: Box<dyn io::Write>,
}
impl NumberedOut {
    fn new() -> NumberedOut {
        NumberedOut {
            n: 0,
            beginning_line: true,
            output: Box::new(io::BufWriter::new(io::stdout())),
        }
    }

    fn print_number(&mut self) -> Result<usize, io::Error> {
        self.n += 1;
        self.beginning_line = true;
        self.output.write(format!("{:6} ", self.n).as_bytes())
    }
}
impl io::Write for NumberedOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        for byte in buf {
            if self.beginning_line {
                self.print_number()?;
            }
            if *byte == b'\n' {
                self.beginning_line = true;
            } else {
                self.beginning_line = false;
            }

            self.output.write_all(&[*byte][..])?;

            if *byte == b'\n' {
                // Flush at the end of each line so if the user is
                // typing input on stdin, they see the numbered output
                // right away (even though output is buffered).
                // Mimics GNU cat.
                self.output.flush()?;
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.output.flush()
    }
}

// copy_to opens a file and copies it to the provided output.
fn copy_to(filename: &str, output: &mut dyn io::Write) -> Result<(), CatError> {
    match File::open(filename) {
        Ok(mut file) => copy_file_to(filename, &mut file, output),
        Err(e) => Err(CatError {
            filename: filename.to_string(),
            message: e.to_string(),
        }),
    }
}

struct CatError {
    filename: String,
    message: String,
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.filename, self.message)
    }
}

// copy_file_to copies bytes from the provided Read object to a Write object.
// Errors will be prefixed with the provided filename.
fn copy_file_to(
    filename: &str,
    input: &mut dyn io::Read,
    output: &mut dyn io::Write,
) -> Result<(), CatError> {
    match io::copy(input, output) {
        Ok(_) => Ok(()),
        Err(e) => Err(CatError {
            filename: filename.to_string(),
            message: e.to_string(),
        }),
    }
}
