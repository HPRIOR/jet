use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use tempfile::{tempdir, TempDir};

pub fn create_temp_flat_app_dir(app_ext: &[String]) -> Result<TempDir, Box<dyn Error>> {
    let dir = tempdir()?;
    let mut increment = 0;
    for ext in app_ext {
        let file_path = dir.path().join(String::from(format!("test{}.", increment.to_string())) + ext);
        increment += 1;
        File::create(file_path).unwrap();
    }
    Ok(dir)
}

pub fn create_temp_nested_app_dir(app_ext: &[&str]) -> Result<PathBuf, Box<dyn Error>> {
    panic!("");
}


