// main.rs
// cargo run -p step_00

// ! transitioning to production code
// The error in tooling/my_lib.rs is still a custom str/String

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod tooling;

use crate::tooling::my_lib;

fn main() -> Result<()> {
    let files = my_lib::list_files("./02_production/00_project/empty")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
