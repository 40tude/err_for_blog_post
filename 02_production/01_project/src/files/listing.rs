// listing.rs

// Refer through the export at crate root
// pub fn list_files(path: &str) -> crate::Result<Vec<String>> {
//
// refer directly to the original module path (no need for the re-export)
// pub fn list_files(path: &str) -> crate::error::Result<Vec<String>> {
//
use crate::Result; // uses the re-export from the root
pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        return Err("Cannot list empty folder.".into());
    }
    Ok(files)
}
