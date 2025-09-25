// mod.rs.01

use crate::{Error, Result}; // now we need Result AND Error

pub fn list_files(path: &str) -> Result<Vec<String>> {
    // add pub
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        // return Err("Cannot list empty folder.".into());
        return Err(Error::CantListEmptyFolder);
    }
    Ok(files)
}

// TODO : create a test
#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = std::result::Result<T, Error>;

    use super::*; // This will overwrite the 2 line above ???

    #[test]
    fn test_name() -> Result<()> {
        // Setup

        // Exec

        // Check
    }
}
