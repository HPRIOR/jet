use std::collections::HashMap;
use crate::jet_brains_app::JetBrainsApp;

#[derive(Debug, PartialEq)]
pub struct AppPoints<'a> {
    pub app_points: u32,
    pub ext_score: HashMap<&'a str, u32>,
}


impl<'a> AppPoints<'a> {
    pub fn new(app: &'a JetBrainsApp) -> Self {
        let ext_score = AppPoints::app_ext_hashmaps(app);
        AppPoints { app_points: 0, ext_score }
    }
    
    fn app_ext_hashmaps(app: &'a JetBrainsApp) -> HashMap<&str, u32> {
        match app {
            JetBrainsApp::Clion => {
                vec![("c",0), ("cpp", 0), ("rs", 0), ("h", 0), ("sql", 0)].into_iter().collect()
            }
            JetBrainsApp::Datagrip => {
                vec![("db", 0), ("dbf", 0), ("sql", 0)].into_iter().collect()
            }
            JetBrainsApp::Intellij => {
                vec![("groovy", 0), ("java", 0), ("jar", 0), ("kt", 0), ("sql", 0)].into_iter().collect()
            }
            JetBrainsApp::Pycharm => {
                vec![("py", 0), ("sql", 0)].into_iter().collect()
            }
            JetBrainsApp::Rider => {
                vec![("cs", 0), ("ashx", 0), ("asp", 0), ("asmx", 0), ("aspx", 0), ("asx", 0), ("axd", 0), ("vb", 0), ("sql", 0)]
                    .into_iter()
                    .collect()
            }
            JetBrainsApp::Webstorm => {
                vec![("ts", 0), ("js", 0), ("html", 0), ("css", 0), ("sql", 0)].into_iter().collect()
            }
        }
    }
}

