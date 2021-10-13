use std::process;

use config::get_apps;

use crate::app_choice::get_selected_app;
use crate::execute_command::open_jetbrains_app;
use crate::file_count::get_app_points;
use crate::generate_app_selection::get_app_with_most_ext;
use crate::path_getter::get_path;

mod app_choice;
mod config;
mod execute_command;
mod file_count;
mod generate_app_selection;
mod jet_brains_app;
mod path_getter;

pub fn run(){
    let current_directory = path_getter::get_path();
    println!("{:?}", current_directory);

    let jet_brains_apps = get_apps(&[
        "rider", "intellij", "clion", "datagrip", "pycharm", "webstorm",
    ]);

    let app_points = get_app_points(&current_directory, &jet_brains_apps);

    match app_points {
        Ok(points) => {
            match get_app_with_most_ext(&points) {
                Ok(generated_app) => {
                    open_jetbrains_app(&current_directory, generated_app);
                },
                Err(inconclusive_message) => {
                    println!("{}", inconclusive_message);
                    let selected_app = get_selected_app(&jet_brains_apps);
                    open_jetbrains_app(&current_directory, selected_app);
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