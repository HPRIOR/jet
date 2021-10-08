mod jet_brains_app;
mod config;
mod ui;
mod execute_command;
mod file_count;
mod choose_app;

use std::process;
use structopt::StructOpt;
use config::get_apps;
use crate::choose_app::get_app_with_most_ext;
use crate::execute_command::open_jetbrains_app;
use crate::file_count::get_app_points;
use crate::ui::display_ui;

fn main() {
    let args: ProcessArgs = ProcessArgs::from_args();
    let path_str = args.file_path.to_str().unwrap_or_else(|| {
        eprintln!("unable to parse 'open' argument");
        process::exit(1);
    });
    let mut current_directory = std::env::current_dir().unwrap();
    current_directory.push(path_str);

    let jet_brains_apps =
        get_apps(&vec!["rider", "intellij", "clion", "datagrip", "pycharm", "webstorm"]);

    let app_points =
        get_app_points(&current_directory, &jet_brains_apps);

    match app_points {
        Ok(points) => {
            match get_app_with_most_ext(&points) {
                Ok(app) => { open_jetbrains_app(&current_directory, app); }
                Err(err_msg) => {
                    eprint!("{}", err_msg);
                    display_ui(&jet_brains_apps, &current_directory);
                }
            };
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
    process::exit(0);
}


#[derive(StructOpt, Debug)]
struct ProcessArgs {
    #[structopt(parse(from_os_str), short = "o", long = "open")]
    file_path: std::path::PathBuf,
}
