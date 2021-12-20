use std::collections::HashMap;
use std::error::Error;

use crate::jet_brains_app::JetBrainsApp;

struct AppPoints<'a> {
    app: &'a JetBrainsApp,
    points: u32,
}

pub fn get_app_with_most_ext<'a>(
    app_points: &HashMap<&'a JetBrainsApp, u32>,
) -> Result<&'a JetBrainsApp, &'a str> {
    let sorted_apps = sort_apps(&app_points.clone());
    let top_app = &sorted_apps[sorted_apps.len() - 1];
    let next_app = &sorted_apps[sorted_apps.len() - 2];

    let app_point_tie = top_app.points == next_app.points;
    if app_point_tie {
        Err("Search inconclusive")
    } else {
        Ok(top_app.app)
    }
}

fn sort_apps<'a>(app_points: &HashMap<&'a JetBrainsApp, u32>) -> Vec<AppPoints<'a>> {
    let mut key_vals: Vec<AppPoints> = app_points
        .iter()
        .map(|(key, val)| AppPoints {
            app: key,
            points: *val,
        })
        .collect();
    key_vals.sort_by_key(|ap| ap.points);
    key_vals
}

#[cfg(test)]
mod tests {
    mod test_sort_apps {
        use std::collections::HashMap;

        use crate::jet_brains_app::JetBrainsApp;
        use crate::top_app_calc::sort_apps;

        #[test]
        fn apps_are_sorted_by_points() {
            // collect utilised the FromIterator to generate new collections from iterators
            // hashmap implements this trait
            let unsorted_apps: HashMap<_, u32> = vec![
                (&JetBrainsApp::Clion, 1),
                (&JetBrainsApp::Pycharm, 5),
                (&JetBrainsApp::Intellij, 2),
            ]
                .into_iter()
                .collect();
            let sut = sort_apps(&unsorted_apps);
            assert_eq!(*sut[0].app, JetBrainsApp::Clion);
            assert_eq!(*sut[1].app, JetBrainsApp::Intellij);
            assert_eq!(*sut[2].app, JetBrainsApp::Pycharm);
        }

        #[test]
        fn apps_are_sorted_by_points_and_points_are_valid() {
            let unsorted_apps: HashMap<_, u32> = vec![
                (&JetBrainsApp::Clion, 1),
                (&JetBrainsApp::Pycharm, 5),
                (&JetBrainsApp::Intellij, 2),
            ]
                .into_iter()
                .collect();
            let sut = sort_apps(&unsorted_apps);
            assert_eq!(sut[0].points, 1);
            assert_eq!(sut[1].points, 2);
            assert_eq!(sut[2].points, 5);
        }
    }
}
