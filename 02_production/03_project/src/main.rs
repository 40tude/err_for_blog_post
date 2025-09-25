// main.rs
// cargo run -p step_03

// ! specific & strict errors
// Remove the   Error::Custom               variant in error.rs
// Remove the impl Error section as well in error.rs
// Add the      Error::CantListEmptyFolder  specific variant


pub use self::error::{Error, Result};

mod error;
mod tooling;

use crate::tooling::my_lib;

fn main() -> Result<()> {
    let files = my_lib::list_files("./02_production/03_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
