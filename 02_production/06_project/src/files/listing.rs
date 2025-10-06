// listing.rs

use crate::{Error, Result};

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        return Err(Error::CantListEmptyFolder);
    }
    Ok(files)
}

#[cfg(test)]
mod test {

    use super::*;

    // ! cwd is 06_project/ NOT 018_err_for_blog_post/ (workspace)
    #[test]
    fn test_empty_folder() {
        let result = list_files("./empty");
        assert!(matches!(result, Err(Error::CantListEmptyFolder)));
    }

    #[test]
    fn test_non_existing_folder() {
        let result = list_files("./non_existent_folder");
        match result {
            Err(Error::Io(_)) => {} // ok, this is an I/O error
            other => panic!("Expected Error::Io, got {:?}", other),
        }
    }

    #[test]
    fn test_current_folder_contains_expected_files_v1() {
        let result = list_files(".").expect("Should list current directory");
        assert_eq!(result, vec!["Cargo.lock", "Cargo.toml"]);
    }

    // Cannot be sure of the order => sort
    #[test]
    fn test_current_folder_contains_expected_files_v2() {
        let mut files = list_files(".").expect("Should list current directory");
        files.sort();
        let mut expected = vec!["Cargo.lock".to_string(), "Cargo.toml".to_string()];
        expected.sort();
        assert_eq!(files, expected);
    }

    // Cannot be sure of the order
    // Cannot be sure other files are not added
    // Just checks both files are present
    #[test]
    fn test_current_folder_contains_expected_files_v3() {
        let files = list_files(".").expect("Should list current directory");
        assert!(files.contains(&"Cargo.toml".to_string()));
        assert!(files.contains(&"Cargo.lock".to_string()));
    }
}
