// main.rs
// cargo run -p step_04

// ! Add testing in listing.rs

use step_05::Result; // uses the re-export from the lib
use step_05::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/02_project/empty")?;
    println!("{files:#?}");
    Ok(())
}
