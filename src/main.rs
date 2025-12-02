use std::env;
use std::error::Error;
use std::fs;
use std::process;

use tolgrep::{search, search_case_insensitive};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let mut ignore_case_option: bool = false;

        if args.len() > 3 {
            let third_arg = args[3].clone();

            ignore_case_option = if third_arg.to_lowercase() == "i" {
                true
            } else {
                false
            }
        }

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
