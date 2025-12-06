use std::env;
use std::error::Error;
use std::fs;
use std::process;

use tolgrep::{search, search_case_insensitive};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1)
    });

    if let Err(err) = run(config) {
        eprintln!("Application Error: {err}");
        process::exit(1);
    }

    Ok(())
}

/// A successful parsing contains the `query` argument and the `file_path`.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub ignore_case_option: bool,
}

impl Config {
    ///
    /// Creates a new instance of a `Config` from `String` slice
    fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        // Go over the first item which is the name of the program
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not provided"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path is not provided"),
        };

        let ignore_case_option = match args.next() {
            Some(arg) => {
                if arg.to_lowercase() == "i" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            ignore_case_option,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case_option || config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for line in results {
        println!("{}. {}", line.index, line.line);
    }

    Ok(())
}
