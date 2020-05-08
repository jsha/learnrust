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
                    eprintln!("{}", e);
                    exit_status = 1;
                }
            }
        }
    }
    process::exit(exit_status);
}

struct NumberedOut {
    n: i64,
    mid_line: bool,
}
impl NumberedOut {
    fn new() -> NumberedOut {
        NumberedOut {
            n: 0,
            mid_line: false,
        }
    }

    fn print_number(&mut self) -> Result<usize, io::Error> {
        self.n += 1;
        self.mid_line = false;
        io::stdout().write(format!("{} ", self.n).as_bytes())
    }
}
impl Write for NumberedOut {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        if !self.mid_line {
            self.print_number().unwrap();
        }
        let mut chunks = buf.split(|x| *x == '\n' as u8);
        io::stdout().write(chunks.next().unwrap())?;

        for line in chunks {
            io::stdout().write(&['\n' as u8; 1])?;
            self.print_number().unwrap();
            io::stdout().write(line)?;
        }

        if *buf.last().unwrap() == '\n' as u8 {
            io::stdout().write(&['\n' as u8; 1])?;
            self.mid_line = false;
        } else {
            self.mid_line = true;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        io::stdout().flush()
    }
}
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

fn copy_file_to<R, W>(filename: &str, input: R, output: &mut W) -> Result<(), CatError>
where
    R: io::Read,
    W: io::Write,
{
    let mut reader = io::BufReader::new(input);
    let mut buffer = [0; 10];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(n) => match output.write(&buffer[..n]) {
                Ok(_) => {}
                Err(e) => {
                    return Err(CatError {
                        filename: filename.to_string(),
                        message: e.to_string(),
                    })
                }
            },
            Err(e) => {
                return Err(CatError {
                    filename: filename.to_string(),
                    message: e.to_string(),
                })
            }
        }
    }
}
