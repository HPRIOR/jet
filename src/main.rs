mod jet_brains_app;
mod config;
mod ui;
mod execute_command;

use std::error::Error;
use std::process;
use structopt::StructOpt;
use config::get_apps;
use crate::ui::display_ui;

fn main() -> Result<(), Box<dyn Error>> {
    let args: ProcessArgs = ProcessArgs::from_args();
    let path_str = args.file_path.to_str().unwrap_or_else(|| {
        eprintln!("unable to parse 'open' argument");
        process::exit(1);
    });

    let installed_apps = vec!["rider", "intellij", "clion", "datagrip", "pycharm", "webstorm"];
    let jet_brains_apps = get_apps(&installed_apps);

    display_ui(&jet_brains_apps, path_str);

    Ok(())
}


#[derive(StructOpt, Debug)]
struct ProcessArgs {
    #[structopt(parse(from_os_str), short = "o", long = "open")]
    file_path: std::path::PathBuf,
}
