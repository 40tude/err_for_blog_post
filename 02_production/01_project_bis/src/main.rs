// main.rs
// cargo run -p step_01

// ! create and use error.rs as a module
// The errors are more structured
// Code in my_lib.rs is not impacted

// This is a re-export from the crate root.
// It makes crate::Error and crate::Result available as if they were defined at the root.
// In a library crate, this also exposes them to other crates (my_crate::Result).
// In a binary crate, nothing external can depend on it, but inside the crate graph it lets all your modules write `crate::Result` instead of `crate::error::Result`
// pub use self::error::{Error, Result}; // re-export to crate root

// use self::error::Result; // here self = crate because main.rs is the root module
// use crate::error::Result; // here self = crate because main.rs is the root module

use step_01_bis::Result; // uses the re-export from the root
use step_01_bis::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/01_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
