use andersonpgrep::Config;
use std::env;
use std::io;
use std::process;

use andersonpgrep::{self};

fn main() -> Result<(), io::Error> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments. {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(er) = andersonpgrep::run(config) {
        eprintln!("Application error {er}");
        process::exit(-1);
    }

    Ok(())
}

// https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
