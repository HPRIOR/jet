mod jet_brains_app;
mod config;
mod ui;
mod execute_command;
mod file_count;

use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use config::get_apps;
use crate::file_count::get_app_points;

fn main() {
    // let args: ProcessArgs = ProcessArgs::from_args();
    // let path_str = args.file_path.to_str().unwrap_or_else(|| {
    //     eprintln!("unable to parse 'open' argument");
    //     process::exit(1);
    // });

    let installed_apps = vec!["rider", "intellij", "clion", "datagrip", "pycharm", "webstorm"];
    let jet_brains_apps = get_apps(&installed_apps);

    // display_ui(&jet_brains_apps, path_str);
    let mut current_directory = std::env::current_dir().unwrap();
    current_directory.push("resources/test_single_files");
    let path: PathBuf = current_directory;
    let result = get_app_points(&path, &jet_brains_apps);
    match result {
        Ok(_) => { println!("success") }
        Err(e) => { println!("{}", e) }
    };
    process::exit(0);
}


#[derive(StructOpt, Debug)]
struct ProcessArgs {
    #[structopt(parse(from_os_str), short = "o", long = "open")]
    file_path: std::path::PathBuf,
}
