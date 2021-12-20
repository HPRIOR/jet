use crate::AppPoints::AppPoints;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

use crate::jet_brains_app::JetBrainsApp;
use crate::jet_brains_app::JetBrainsApp::{Clion, Datagrip, Intellij, Pycharm, Rider, Webstorm};

pub fn get_app_points<'a>(
    file_path: &Path,
    apps: &'a [JetBrainsApp],
) -> Result<HashMap<&'a JetBrainsApp, AppPoints<'a>>, Box<dyn Error>> {
    let mut app_points = get_app_points_map(apps);
    let app_ext = get_app_ext_hashmaps(apps);
    let files = fs::read_dir(file_path)?;
    recurse_directories(&mut app_points, &app_ext, files)?;
    Ok(app_points)
}

fn recurse_directories<'a>(
    app_points: &mut HashMap<&'a JetBrainsApp, AppPoints>,
    app_ext: &HashMap<&'a JetBrainsApp, HashSet<&'static str>>,
    files: ReadDir,
) -> Result<(), Box<dyn Error>> {
    for dir_result in files {
        let dir_entry = dir_result?;
        let meta_data = dir_entry.metadata()?;
        if meta_data.is_file() {
            update_count_with_file_ext(app_points, app_ext, &dir_entry)
        }
        if meta_data.is_dir() {
            if let Ok(read_dir) = fs::read_dir(dir_entry.path()) {
                recurse_directories(app_points, app_ext, read_dir).unwrap()
            }
        }
    }
    Ok(())
}

fn update_count_with_file_ext<'a>(
    app_points: &mut HashMap<&'a JetBrainsApp, AppPoints>,
    app_ext: &HashMap<&'a JetBrainsApp, HashSet<&'static str>>,
    dir_entry: &DirEntry,
) {
    let entry_file_name: Vec<String> = dir_entry
        .file_name()
        .to_str()
        .unwrap()
        .split('.')
        .map(|s| s.to_string())
        .collect();

    if let Some(ext) = entry_file_name.last() {
        modify_app_points(app_points, app_ext, ext)
    }
}

// todo redo this with get_or_else
fn get_app_points_map(apps: &[JetBrainsApp]) -> HashMap<&JetBrainsApp, AppPoints> {
    let mut app_points: HashMap<&JetBrainsApp, AppPoints> = HashMap::new();
    apps.iter().for_each(|app| {
        app_points.insert(app, AppPoints::new(app));
    });
    app_points
}

fn modify_app_points<'a>(
    app_points: &mut HashMap<&'a JetBrainsApp, AppPoints>,
    app_ext_map: &HashMap<&'a JetBrainsApp, HashSet<&'static str>>,
    ext: &str,
) {
    for (app, extensions) in app_ext_map {
        if extensions.contains(ext) {
            let app_points = app_points.get_mut(*app).unwrap();
            app_points.app_points += 1;
            *app_points.ext_score.get_mut(ext).unwrap() += 1;
        }
    }
}

fn get_app_ext_hashmaps(apps: &[JetBrainsApp]) -> HashMap<&JetBrainsApp, HashSet<&'static str>> {
    let mut result = HashMap::new();
    for app in apps {
        match app {
            Clion => {
                result.insert(
                    app,
                    vec!["c", "cpp", "rs", "h", "sql"].into_iter().collect(),
                );
            }
            Datagrip => {
                result.insert(app, vec!["db", "dbf", "sql"].into_iter().collect());
            }
            Intellij => {
                result.insert(
                    app,
                    vec!["groovy", "java", "jar", "kt", "sql"]
                        .into_iter()
                        .collect(),
                );
            }
            Pycharm => {
                result.insert(app, vec!["py", "sql"].into_iter().collect());
            }
            Rider => {
                result.insert(
                    app,
                    vec![
                        "cs", "ashx", "asp", "asmx", "aspx", "asx", "axd", "vb", "sql",
                    ]
                    .into_iter()
                    .collect(),
                );
            }
            Webstorm => {
                result.insert(
                    app,
                    vec!["ts", "js", "html", "css", "sql"].into_iter().collect(),
                );
            }
        }
    }
    result
}

#[cfg(test)]
mod file_count_tests {
    mod modify_app_points {
        use crate::dir_scanner::{get_app_ext_hashmaps, get_app_points_map, modify_app_points};
        use crate::jet_brains_app::JetBrainsApp;
        use crate::AppPoints::AppPoints;
        use std::collections::HashMap;

        #[test]
        fn app_points_are_updated_with_correct_ext() {
            let apps = vec![JetBrainsApp::Clion];
            let mut app_points = get_app_points_map(&apps);
            let app_extensions = get_app_ext_hashmaps(&apps);
            modify_app_points(&mut app_points, &app_extensions, "c");

            assert_eq!(
                app_points[&JetBrainsApp::Clion],
                AppPoints {
                    app_points: 1,
                    ext_score: vec![("c",1), ("cpp", 0), ("rs", 0), ("h", 0), ("sql", 0)].into_iter().collect()
                }
            );
        }

        #[test]
        fn app_points_are_not_updated_with_incorrect_ext() {
            let apps = vec![JetBrainsApp::Clion];
            let mut app_points = get_app_points_map(&apps);
            let app_extensions = get_app_ext_hashmaps(&apps);
            modify_app_points(&mut app_points, &app_extensions, "py");

            assert_eq!(
                app_points[&JetBrainsApp::Clion],
                AppPoints {
                    app_points: 0,
                    ext_score: vec![("c",0), ("cpp", 0), ("rs", 0), ("h", 0), ("sql", 0)].into_iter().collect()
                }
            );
        }
    }
}
