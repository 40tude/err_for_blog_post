// main.rs
// cargo run -p step_04

// ! In error.rs : 21:32
// ! Remove Custom(String) variant from Error enum
// ! Remove the Custom region as well
// No longer compile : .into() in return Err("Cannot list empty folder.".into());
// Because it does not take string like anymore
// In listing replace use crate::Result; with use crate::{Error, Result};
// Add CantListEmptyFolder variant to Error enum
// Then replace return Err("Cannot list empty folder.".into());
// with return Err(Error::CantListEmptyFolder);
// Output = "Error: CantListEmptyFolder"

use step_04::Result; // uses the re-export from the lib
use step_04::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/02_project/empty")?;
    println!("{files:#?}");
    Ok(())
}
