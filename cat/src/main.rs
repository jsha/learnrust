use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;
use std::string;

fn main() {
    let mut output: Box<dyn io::Write> = Box::new(io::stdout());
    let show_line_numbers = env::var("NUMBER").unwrap_or("0".to_string());
    if show_line_numbers == "1" {
        output = Box::new(NumberedOut::new());
    }
    process_args(env::args(), &mut output);
}

fn process_args(args: env::Args, output: &mut Box<dyn io::Write>) {
    let mut exit_status = 0;
    if args.len() == 1 {
        match copy_file_to("-", io::stdin(), output) {
            Ok(()) => {}
            Err(e) => {
                output.flush().unwrap();
                eprintln!("{}", e);
                exit_status = 1
            }
        }
    } else {
        for arg in args.skip(1) {
            let result = if arg == "-" {
                copy_file_to("-", io::stdin(), output)
            } else {
                copy_to(&arg, output)
            };
            match result {
                Ok(()) => continue,
                Err(e) => {
                    output.flush().unwrap();
                    eprintln!("{}", e);
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
    output: Box<dyn Write>,
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
        self.output.write(format!("{} ", self.n).as_bytes())
    }
}
impl Write for NumberedOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        for byte in buf {
            if self.beginning_line {
                self.print_number()?;
            }
            if *byte == '\n' as u8 {
                self.beginning_line = true;
            } else {
                self.beginning_line = false;
            }

            self.output.write(&[*byte][..])?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.output.flush()
    }
}

// copy_to opens a file and copies it to the provided output.
fn copy_to<W: io::Write>(filename: &str, output: &mut W) -> Result<(), CatError> {
    match File::open(filename) {
        Ok(file) => copy_file_to(filename, &file, output),
        Err(e) => {
            return Err(CatError {
                filename: filename.to_string(),
                message: e.to_string(),
            })
        }
    }
}

struct CatError {
    filename: string::String,
    message: string::String,
}

impl fmt::Display for CatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.filename, self.message)
    }
}

// copy_file_to copies bytes from the provided Read object to a Write object.
// Errors will be prefixed with the provided filename.
fn copy_file_to<R, W>(filename: &str, input: R, output: &mut W) -> Result<(), CatError>
where
    R: io::Read,
    W: io::Write,
{
    let mut reader = io::BufReader::new(input);
    let mut buffer = [0; 1000];
    let err = |msg: &str| -> std::result::Result<(), CatError> {
        Err(CatError {
            filename: filename.to_string(),
            message: msg.to_string(),
        })
    };
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => match output.write(&buffer[..n]) {
                Ok(0) => return err("failed to write to output"),
                Ok(_) => {}
                Err(e) => return err(&e.to_string()),
            },
            Err(e) => return err(&e.to_string()),
        }
    }
}
