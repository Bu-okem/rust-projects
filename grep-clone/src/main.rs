use std::env::args;
use std::error::Error;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use std::process::exit;

struct Config {
    query: String,
    directory_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            directory_path: args[2].clone(),
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

fn search_in_dir(path: &Path, query: &str) -> Result<(), Box<dyn Error>> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let folder_path = entry.path();
        if folder_path.is_file() {
            let file = match read_to_string(&folder_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Skipping [{}]: {}", folder_path.display(), e); // skip invalid UTF-8
                    continue;
                }
            };
            let lines_with_query = search(&file, query);
            for line in lines_with_query {
                println!("[{}]: {}", folder_path.display(), line)
            }
        } else {
            search_in_dir(&folder_path, query)?;
        }
    }

    Ok(())
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    search_in_dir(Path::new(&config.directory_path), &config.query)?;

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
