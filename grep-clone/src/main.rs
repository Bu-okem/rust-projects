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

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = read_to_string(&config.file_path)?;

    let content = file.lines();

    let query = &config.query.to_lowercase();

    for line in content {
        if line.to_lowercase().contains(query) {
            println!("Contained in: {}", line);
        }
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
