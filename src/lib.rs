use std::error::Error;
use std::path::{Path, PathBuf};

use config::get_apps;

use crate::app_choice::get_selected_app;
use crate::dir_scanner::get_app_points;
use crate::top_app_calc::get_app_with_most_ext;

mod app_choice;
mod config;
mod dir_scanner;
mod top_app_calc;
mod jet_brains_app;
mod AppPoints;


pub fn jet(path_arg: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let jet_brains_apps = get_apps(&[
        "rider", "intellij", "clion", "datagrip", "pycharm", "webstorm",
    ]);


    let app_points = get_app_points(path_arg, &jet_brains_apps)?;
    println!("{:?}", app_points);
    Ok(PathBuf::new())
    //  return Ok(if let Ok(generated_app) = get_app_with_most_ext(&app_points) {
    //     generated_app.get_path()
    // } else {
    //     println!("Scan inconclusive, please choose app: ");
    //     get_selected_app(&jet_brains_apps).get_path()
    // });
}
