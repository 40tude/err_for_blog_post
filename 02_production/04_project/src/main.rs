// 04_prod_code.rs  to be renamed   main.rs
// work with
//      error.rs.03 to be renamed   error.rs
//      mod.rs.02   to be renamed   mod.rs

// cargo run

// ! specific & strict errors
// Remove the   Error::Custom               variant
// Add the      Error::CantListEmptyFolder  specific variant

mod error;
mod fs;

pub use self::error::{Error, Result};

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files("./empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
