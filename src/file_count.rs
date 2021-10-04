use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn count_files(file_path: &PathBuf) -> Result<HashMap<&str, u32>, std::io::Error>{
    let mut file_count = HashMap::new();
    let files = fs::read_dir(file_path)?;
    for file in files{
        let entry = file.unwrap();
    };
    Ok(file_count)
}

#[cfg(test)]
mod file_count_tests {
    use std::path::PathBuf;
    use crate::file_count::count_files;

    #[test]
    fn sanity_test(){
        let  path: PathBuf = ["resources", "test_single_file"].iter().collect();
        let sut = count_files(&path);
    }
    #[test]
    fn will_count_single_file(){
        let  path: PathBuf = ["resources", "test_single_file"].iter().collect();
        let sut= count_files(&path).unwrap();
        assert!(sut.len() > 0);
        sut.into_iter().for_each(|(key, val)|{
            assert_eq!(val, 1);
        })
    }
}