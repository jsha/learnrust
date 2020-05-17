use std::env;
use std::io;

fn main() {
    let conf = if let Some(conf) = process_args() {
        conf
    } else {
        eprintln!("Usage: {} host port", env::args().next().unwrap());
        std::process::exit(1);
    };

    if let Err(e) = connect(&conf.host, &conf.port) {
        eprintln!("connecting to {}:{}: {}", conf.host, conf.port, e);
        std::process::exit(1);
    }
}

fn connect(host: &str, port: &str) -> io::Result<()> {
    let mut stream = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
    let mut stream2 = stream.try_clone()?;

    let receiving = std::thread::spawn(move || {
        if let Err(e) = io::copy(&mut io::stdin(), &mut stream) {
            eprintln!("sending: {}", e)
        }
    });
    let sending = std::thread::spawn(move || {
        if let Err(e) = io::copy(&mut stream2, &mut io::stdout()) {
            eprintln!("receiving: {}", e)
        }
    });
    receiving.join().unwrap();
    sending.join().unwrap();
    Ok(())
}

fn process_args() -> Option<Config> {
    let mut conf = Config::default();
    conf.host = env::args().nth(1)?;
    conf.port = env::args().nth(2)?;
    Some(conf)
}

#[derive(Default)]
struct Config {
    host: String,
    port: String,
}
