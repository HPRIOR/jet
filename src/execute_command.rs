use std::process::{Command, Output};
use crate::jet_brains_app::JetBrainsApp;

pub fn open_jetbrains_app(path_str: &str, app: &JetBrainsApp) -> Output {
    let mut full_path_string = std::env::current_dir()
        .unwrap();
    full_path_string.push(path_str);
    let full_path_string = full_path_string.to_str().unwrap_or_else(|| "");
    Command::new("open")
        .arg("-na")
        .arg(app.get_path().to_str().unwrap_or_else(|| ""))
        .args(["--args", format!("\"{}\"", full_path_string).as_str(), "nosplash"])
        .output()
        .expect("failed to execute command")
}
