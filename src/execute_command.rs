use std::path::Path;
use std::process::{Command, Output};

pub fn open_jetbrains_app(project_path: &Path, app_path: &Path) -> Output {
    Command::new("open")
        .arg("-na")
        .arg(app_path.to_str().unwrap_or(""))
        .args([
            "--args",
            format!("\"{}\"", project_path.to_str().unwrap()).as_str(),
            "nosplash",
        ])
        .output()
        .expect("failed to execute command")
}
