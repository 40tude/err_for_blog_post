// main.rs
// cargo run -p step_02

// ! Add lib.rs

use step_02::Result; // uses the re-export from the lib.rs
use step_02::files::listing;

// use my_super_lib::Result; // uses the re-export from the lib.rs
// use my_super_lib::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files(".")?;
    println!("{files:#?}");

    let files = listing::list_files("./non_existent_folder")?;
    println!("{files:#?}");

    Ok(())
}
