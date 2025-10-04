// main.rs
// cargo run -p step_03
// cargo add thiserror --package step_03

// ! Add thiserror

use step_03::Result; // uses the re-export from the lib
use step_03::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files(".")?;
    println!("{files:#?}");

    // let files = listing::list_files("./02_production/03_project/empty")?;
    // println!("{files:#?}");

    let files = listing::list_files("./non_existent_folder")?;
    println!("{files:#?}");

    Ok(())
}
