use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use crate::jet_brains_app::JetBrainsApp;
use crate::jet_brains_app::JetBrainsApp::{Clion, Datagrip, Intellij, Pycharm, Rider, Webstorm}; // 0.17.1

pub fn get_app_points<'a>(file_path: &PathBuf, apps: &'a Vec<JetBrainsApp>) -> Result<HashMap<&'a JetBrainsApp, u32>, Box<dyn Error>> {
    let mut app_points = get_app_points_map(apps);
    let app_ext = get_app_ext_hashmaps(apps);
    let files = fs::read_dir(file_path)?;
    for file in files {
        let entry = file?.file_name();
        let entry_str = entry.to_str().unwrap().split(".");
        entry_str.for_each(|file_str| modify_app_points(&mut app_points, &app_ext, file_str))
    };
    Ok(app_points)
}
fn get_app_points_map(apps: &Vec<JetBrainsApp>) -> HashMap<&JetBrainsApp, u32> {
    let mut app_points: HashMap<&JetBrainsApp, u32> = HashMap::new();
    apps.iter().for_each(|app| {
        app_points.insert(app, 0);
    });
    app_points
}
fn modify_app_points<'a>(
    app_points: &mut HashMap<&'a JetBrainsApp, u32>,
    app_ext_map: &HashMap<&'a JetBrainsApp, HashSet<&'static str>>,
    ext: &str) -> () {
    for (app, extensions) in app_ext_map{
        if extensions.contains(ext) {
            *app_points.get_mut(*app).unwrap() += 1;
        }
    }
}

fn get_app_ext_hashmaps(apps: &Vec<JetBrainsApp>) -> HashMap<&JetBrainsApp, HashSet<&'static str>> {
    let mut result = HashMap::new();
    for app in apps {
        match app {
            Clion => {
                result.insert(app, vec![
                    "c",
                    "cpp",
                    "rs",
                    "h",
                ].into_iter().collect());
            }
            Datagrip => {
                result.insert(app, vec![
                    "db",
                    "dbf",
                    "sql",
                ].into_iter().collect());
            }
            Intellij => {
                result.insert(app, vec![
                    "groovy",
                    "java",
                    "jar",
                    "kt",
                ].into_iter().collect());
            }
            Pycharm => {
                result.insert(app, vec![
                    "py"
                ].into_iter().collect());
            }
            Rider => {
                result.insert(app, vec![
                    "cs",
                    "ashx",
                    "asp",
                    "asmx",
                    "aspx",
                    "asx",
                    "axd",
                    "vb"
                ].into_iter().collect());
            }
            Webstorm => {
                result.insert(app, vec![
                    "ts",
                    "js",
                    "html",
                    "css",
                ].into_iter().collect());
            }
        }
    }
    result
}

#[cfg(test)]
mod file_count_tests {
    mod modify_app_points {
        use std::collections::HashMap;
        use crate::file_count::{get_app_ext_hashmaps, get_app_points, get_app_points_map, modify_app_points};
        use crate::jet_brains_app::JetBrainsApp;


        #[test]
        fn app_points_are_updated_with_correct_ext() {
            let apps = vec![JetBrainsApp::Clion];
            let mut app_points = get_app_points_map(&apps);
            let app_extensions = get_app_ext_hashmaps(&apps);
            modify_app_points(&mut app_points, &app_extensions, "c");

            assert_eq!(app_points[&JetBrainsApp::Clion], 1);
        }

        #[test]
        fn app_points_are_not_updated_with_incorrect_ext() {
            let apps = vec![JetBrainsApp::Clion];
            let mut app_points = get_app_points_map(&apps);
            let app_extensions = get_app_ext_hashmaps(&apps);
            modify_app_points(&mut app_points, &app_extensions, "py");

            assert_eq!(app_points[&JetBrainsApp::Clion], 0);
        }
    }

    mod get_app_points {
        use std::path::PathBuf;
        use crate::file_count::get_app_points;
        use crate::jet_brains_app::JetBrainsApp;

        #[test]
        fn with_only_files_correct_app_points_are_given() {
            let mut file_path = std::env::current_dir().unwrap();
            file_path.push("resources/test_single_files");

            let apps: Vec<JetBrainsApp> = vec!["clion", "pycharm", "rider", "intellij", "datagrip", "webstorm"]
                .into_iter()
                .map(|s| JetBrainsApp::new(s).unwrap())
                .collect();

            let sut = get_app_points(&file_path, &apps).unwrap();
            assert_eq!(sut[&JetBrainsApp::Clion], 4);
            assert_eq!(sut[&JetBrainsApp::Pycharm], 1);
            assert_eq!(sut[&JetBrainsApp::Rider], 8);
            assert_eq!(sut[&JetBrainsApp::Intellij], 4);
            assert_eq!(sut[&JetBrainsApp::Datagrip], 3);
            assert_eq!(sut[&JetBrainsApp::Webstorm], 4);
        }
    }
}