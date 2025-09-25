// main.rs
// cargo run -p step_01

// ! create and use error.rs as a module
// The errors are more structured
// Code in my_lib.rs is not impacted

mod error;
mod tooling;

// self refers to the current module.
// If we are in main.rs, then self refers to the root module of the binary crate
// If we are in a module file (e.g., lib.rs or a submodule), self refers to that module
// The line below is equivalent to : pub use crate::error::{Error, Result};
pub use self::error::{Error, Result}; // reexport

use crate::tooling::my_lib;

fn main() -> Result<()> {
    let files = my_lib::list_files("./02_production/01_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
