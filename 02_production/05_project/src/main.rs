// main.rs
// cargo run -p step_05

// ! In error.rs
// ! Remove Custom(String) variant from Error enum

// No longer compile : .into() in return Err("Cannot list empty folder.".into());
// Because it does not take string like anymore
// In listing replace use crate::Result; with use crate::{Error, Result};
// Add CantListEmptyFolder variant to Error enum
// Then replace return Err("Cannot list empty folder.".into());
// with return Err(Error::CantListEmptyFolder);
// Output = "Error: CantListEmptyFolder"

use step_05::Result; // uses the re-export from the lib
use step_05::files::listing;

fn main() -> Result<()> {
    match listing::list_files(".") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error: {e}"),
    }

    match listing::list_files("./02_production/04_project/empty") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error detected: {e}"),
    }

    match listing::list_files("./non_existent_folder") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error detected: {e}"),
    }

    Ok(())
}
