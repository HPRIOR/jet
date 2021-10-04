enum JetBrainsApp {
    Rider,
    Intellij,
    Clion,
    Datagrip,
    Pycharm,
    Webstorm,
}

impl JetBrainsApp {
    fn new(app: &str) -> Option<JetBrainsApp> {
        match app.to_lowercase().as_str() {
            "rider" => Some(Rider),
            "intellij" => Some(Intellij),
            "clion" => Some(Clion),
            "datagrip" => Some(Datagrip),
            "pycharm" => Some(Pycharm),
            "webstorm" => Some(Webstorm),
            _ => None
        }
    }

    fn get_path(&self) -> PathBuf {
        match self {
            Rider => ["Rider.app"],
            Intellij => ["IntelliJ IDEA.app"],
            Clion => ["Clion.app"],
            Datagrip => ["DataGrip.app"],
            Pycharm => ["PyCharm.app"],
            Webstorm => ["WebStorm.app"],
        }.iter().collect()
    }
}

