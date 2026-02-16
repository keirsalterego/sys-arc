use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Config {
    query: String,
    filname: String,
    invert: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filname: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let invert: bool = env::var("INVERT").is_ok();

        Ok(Config {
            query,
            filname,
            invert,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.filname)?;

    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let is_match = if config.invert {
            line.to_lowercase().contains(&config.query.to_lowercase())
        } else {
            line.contains(&config.query)
        };

        if is_match {
            // Print line number and content
            println!("{}: {}", index + 1, line);
        }
    }

    Ok(())
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
