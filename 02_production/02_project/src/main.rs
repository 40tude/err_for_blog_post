// main.rs
// cargo run -p step_02

// ! specific & strict errors
// Remove the   Error::Custom               variant
// Add the      Error::CantListEmptyFolder  specific variant

pub use self::error::{Error, Result};

mod error;
mod tooling;

use crate::tooling::my_lib;

fn main() -> Result<()> {
    let files = my_lib::list_files("./02_production/02_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
