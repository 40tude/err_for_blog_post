// 02_prod_code.rs  to be renamed   main.rs
// work with
//      error.rs.01 to be renamed   error.rs
//      mod.rs.01   to be renamed   mod.rs

// cargo run

// ! create and use error.rs
// create error.rs as a module

mod error;
mod fs;

// self refers to the current module.
// If you are in main.rs, then self refers to the root module of the binary crate
// If you are in a module file (e.g., lib.rs or a submodule), self refers to that module
// The line below is equivalent to : pub use crate::error::{Error, Result};
pub use self::error::{Error, Result}; // reexport

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
