// main.rs
// cargo run -p step_03

// ! specific & strict errors
// Add testing in tooling/my_lib.rs

pub use self::error::{Error, Result};

mod error;
mod tooling;

use crate::tooling::my_lib;

fn main() -> Result<()> {
    let files = my_lib::list_files("./02_production/03_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
