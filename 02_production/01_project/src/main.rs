// main.rs
// cargo run -p step_01

// ! create error.rs module
// No lib.rs yet, we are building a binary crate

mod error;
mod files;

use crate::error::Result;
use crate::files::listing;

fn main() -> crate::error::Result<()> {
    // fn main() -> Result<()> {
    let files = listing::list_files(".")?;
    println!("{files:#?}");

    let files = listing::list_files("./02_production/01_project/empty")?;
    println!("{files:#?}");
    Ok(())
}
