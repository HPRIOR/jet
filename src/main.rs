use std::process;

use jet::jet;

use crate::execute_command::open_jetbrains_app;

mod execute_command;
mod path_getter;

fn main() {
    let project_path = path_getter::get_project_path();
    let app_path = jet(&project_path);
    match app_path {
        Ok(result) => { open_jetbrains_app(&project_path, &result); }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            process::exit(1);
        }
    }
    process::exit(0);
}
