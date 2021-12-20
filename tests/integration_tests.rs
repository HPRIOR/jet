use std::fs::File;

use tempfile::tempdir;

use common::create_temp_flat_app_dir;
use jet::jet;

mod common;

#[test]
fn sanity_check() {
    assert!(true)
}

#[test]
fn will_open_rider_app() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("cs"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "Rider.app");
    Ok(())
}

#[test]
fn will_open_webstorm() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("html"),
        String::from("ts"),
        String::from("js"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "WebStorm.app");
    Ok(())
}

#[test]
fn will_open_intellij() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("jar"),
        String::from("java"),
        String::from("groovy"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "IntelliJ IDEA.app");
    Ok(())
}


#[test]
fn will_open_clion() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("rs"),
        String::from("c"),
        String::from("cpp"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "Clion.app");
    Ok(())
}

#[test]
fn will_open_datagrip() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("sql"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "DataGrip.app");
    Ok(())
}

#[test]
fn will_open_pycharm() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("py"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "PyCharm.app");
    Ok(())
}


#[test]
fn will_use_second_app_if_sql_ext_not_exclusive() -> Result<(), Box<dyn std::error::Error>> {
    let extensions = [
        String::from("sql"),
        String::from("sql"),
        String::from("cs"),
    ];
    let dir = create_temp_flat_app_dir(&extensions).unwrap();
    let sut = jet(dir.path())?;
    assert_eq!(sut.to_str().unwrap(), "Rider.app");
    Ok(())
}




