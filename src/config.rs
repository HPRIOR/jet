use crate::jet_brains_app::JetBrainsApp;

pub fn get_apps(installed_apps: &Vec<&str>) -> Vec<JetBrainsApp> {
    installed_apps
        .iter()
        .filter_map(|app| JetBrainsApp::new(app))
        .collect()
}


#[cfg(test)]
mod config_tests {
    use crate::config::get_apps;

    #[test]
    fn all_installed_apps_are_valid() {
        let installed_apps = vec!["rider", "intellij", "clion", "datagrip", "pycharm", "webstorm"];
        let sut = get_apps(&installed_apps);
        assert_eq!(installed_apps.len(), sut.len())
    }
}