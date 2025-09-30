// main.rs
// cargo run -p step_00

// ! transitioning to production code
// The error in tooling/my_lib.rs is still a custom str/String

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod files;

// self refers to the current module.
// If we are in main.rs, then self refers to the root module of the binary crate
// If we are in a module file (e.g., lib.rs or a submodule), self refers to that module
// Both lines works
// use self::tooling::my_lib;
use crate::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/00_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
