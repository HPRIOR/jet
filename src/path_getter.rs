use std::path::PathBuf;
use std::process;

pub fn get_path() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        eprintln!("1 argument expected, found {}", args.len());
        process::exit(1);
    }
    let mut current_directory = std::env::current_dir().unwrap();
    if let Some(path_string) = args.get(1) {
        current_directory.push(path_string);
    }
    current_directory
}
