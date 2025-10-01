// main.rs
// cargo run -p step_00

// ! transitioning to production code
// Split existing code in different modules
// The error in files/listing.rs is still a custom str/String

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod files;

// self refers to the current module.
//      If we are in a module file `self` refers to that module in the module tree
//      If we are in `main.rs` (binary crate), then self refers to the root module of the binary crate
// This is why, here, both lines below works
//
// use self::files::listing;
use crate::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/00_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
