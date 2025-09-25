// 03_prod_code.rs  to be renamed   main.rs
// work with
//      error.rs.02 to be renamed   error.rs
//      mod.rs.01   to be renamed   mod.rs

// cargo run

// ! extend the content of error.rs
// derive_more is mandatory
//      cargo add derive_more --features from

mod error;
mod fs;

pub use self::error::{Error, Result};

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
