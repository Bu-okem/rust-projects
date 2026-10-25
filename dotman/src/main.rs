use std::env::args;
use std::os::unix::fs::symlink;
use std::process::exit;
use std::{error::Error, path::Path};

struct Config {
    source_path: String,
    target_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let source_path = Path::new(&args[1]);
        let target_path = Path::new(&args[2]);
        if !Path::exists(source_path) || Path::is_dir(source_path) {
            return Err("The provided path is not valid");
        }

        let target_parent = match target_path.parent() {
            Some(p) => p,
            None => return Err("Target path must include a directory"),
        };

        if !target_parent.exists() {
            return Err("File doesn't exist");
        }

        Ok(Config {
            source_path: args[1].clone(),
            target_path: args[2].clone(),
        })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new(&config.target_path);
    if target_path.exists() {
        std::fs::rename(target_path, target_path.with_extension(".bak"))?;
    }
    symlink(&config.source_path, &config.target_path)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        exit(1);
    });

    run(&config).unwrap_or_else(|err| {
        eprintln!("Application Error: {}", err);
        exit(1);
    })
}
