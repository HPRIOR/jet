use std::path::PathBuf;
use crate::jet_brains_app::JetBrainsApp::{Clion, Datagrip, Intellij, Pycharm, Rider, Webstorm};
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, PartialEq, EnumIter, Eq, Hash, Copy)]
pub enum JetBrainsApp {
    Clion,
    Datagrip,
    Intellij,
    Pycharm,
    Rider,
    Webstorm,
}

impl JetBrainsApp {
    pub fn new(app: &str) -> Option<JetBrainsApp> {
        match app.to_lowercase().as_str() {
            "clion" => Some(Clion),
            "datagrip" => Some(Datagrip),
            "intellij" => Some(Intellij),
            "pycharm" => Some(Pycharm),
            "rider" => Some(Rider),
            "webstorm" => Some(Webstorm),
            _ => None
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self {
            Clion => ["Clion.app"],
            Datagrip => ["DataGrip.app"],
            Intellij => ["IntelliJ IDEA.app"],
            Pycharm => ["PyCharm.app"],
            Rider => ["Rider.app"],
            Webstorm => ["WebStorm.app"],
        }.iter().collect()
    }
}

impl Clone for JetBrainsApp{
    fn clone(&self) -> Self {
        match self {
            Clion => JetBrainsApp::Clion,
            Datagrip => JetBrainsApp::Datagrip,
            Intellij => JetBrainsApp::Intellij,
            Pycharm => JetBrainsApp::Pycharm,
            Rider => JetBrainsApp::Rider,
            Webstorm => JetBrainsApp::Webstorm
        }
    }
}

#[cfg(test)]
mod jet_brains_app_tests{
    use crate::jet_brains_app::JetBrainsApp;

    #[test]
    fn new_will_create_appropriate_enum(){
        let sut = JetBrainsApp::new("clion");
        let app = sut.unwrap();
        assert_eq!(app, JetBrainsApp::Clion)
    }

    #[test]
    fn new_will_accept_uppercase_arg(){
        let sut = JetBrainsApp::new("CLiOn");
        let app = sut.unwrap();
        assert_eq!(app, JetBrainsApp::Clion)
    }

    #[test]
    fn get_path_will_return_correct_path(){
        let sut = JetBrainsApp::new("clion");
        let app = sut.unwrap();
        assert_eq!("Clion.app", app.get_path().to_str().unwrap())
    }

   #[test]
    fn will_return_none_bad_arg(){
       let sut = JetBrainsApp::new("blion");
       assert!(sut.is_none())
   }


}