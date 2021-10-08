use std::path::PathBuf;
use std::process::{Command, Output};

use crate::jet_brains_app::JetBrainsApp;

pub fn open_jetbrains_app(path: &PathBuf, app: &JetBrainsApp) -> Output {
    Command::new("open")
        .arg("-na")
        .arg(app.get_path().to_str().unwrap_or_else(|| ""))
        .args(["--args", format!("\"{}\"", path.to_str().unwrap()).as_str(), "nosplash"])
        .output()
        .expect("failed to execute command")
}
