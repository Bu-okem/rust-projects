use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Problem parsing arguments: not enough arguments");
        exit(1);
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    let file = read_to_string(file_path).expect("Failed to read file");

    let content = file.lines();

    for line in content {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("Contained in: {}", line);
        }
    }
}
