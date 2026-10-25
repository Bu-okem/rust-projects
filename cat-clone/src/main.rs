use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

struct ArgumentError {
    message: String,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ArgumentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        let err = Box::new(ArgumentError {
            message: "Problem parsing arguments: Not enough arguments".to_string(),
        });
        return Err(err);
    } else {
        let content = read_to_string(&args[1])?;

        println!("{}", content);
    }

    Ok(())
}

/*
* OR
*
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Problem parsing arguments: Not enough arguments");
    } else {
        let content = read_to_string(&args[1]);

        match content {
            Ok(string) => println!("{}", string),
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    }
}
*/
