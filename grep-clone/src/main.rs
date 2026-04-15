use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            eprintln!("Problem parsing arguments: not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn search<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let lines = content.lines();
    let query = &query.to_lowercase();
    let contains_query = lines
        .filter(|line| line.to_lowercase().contains(query))
        .collect();

    contains_query
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(&config.file_path)?;

    let lines_with_query = search(&file, &config.query);

    for line in lines_with_query {
        println!("Contained in: {}", line);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1);
    });

    if let Err(e) = run(&config) {
        eprintln!("Application error: {}", e);
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(content, query));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "Rust:\nsafe, fast, productive.";

        assert_eq!(vec!["Rust:"], search(content, query));
    }
}
