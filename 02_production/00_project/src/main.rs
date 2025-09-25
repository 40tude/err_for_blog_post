// main.rs
// cargo run -p step_00

// ! transitioning to production code

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod file_utility;

use crate::file_utility::lf;

fn main() -> Result<()> {
    let files = lf::list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
