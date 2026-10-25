mod arg_error;
use arg_error::ArgumentError;
use std::env::args;
use std::error::Error;
use std::fs::read_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        return Err(Box::new(ArgumentError::new(String::from(
            "Not enough arguments",
        ))));
    }
    let path = Path::new(&args[1]);
    if !Path::exists(path) || !Path::is_dir(path) {
        return Err(Box::new(ArgumentError::new(String::from(
            "The provided path is not a valid directory",
        ))));
    } else {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let extension = path.extension();
                let extension_string: Option<String> = match extension {
                    Some(os_str) => match os_str.to_str() {
                        Some(s) => Some(s.to_string()),
                        None => None,
                    },
                    None => None,
                };

                let new_path: String = match extension_string {
                    Some(s) => String::from(s),
                    None => String::from("other"),
                };

                let parent = path.parent();
                let file = path.file_name();

                let target_dir = parent.unwrap().join(new_path);
                let target_file = target_dir.join(file.unwrap());

                std::fs::create_dir_all(&target_dir)?;

                std::fs::rename(path, target_file)?;
            }
        }
        Ok(())
    }
}
