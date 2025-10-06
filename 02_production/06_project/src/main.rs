// main.rs
// cargo run -p step_06

// ! In error.rs
// ! Remove Custom(String) variant from Error enum

use step_06::Result;
use step_06::files::listing;

fn main() -> Result<()> {
    match listing::list_files(".") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error: {e}"),
    }

    match listing::list_files("./02_production/06_project/empty") {
        Ok(files) => println!("Files found   : {files:#?}"),
        // Err(e) => println!("Error detected: {e}"),
        Err(e) => println!("Error detected: {e:?}"),
    }

    match listing::list_files("./non_existent_folder") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error detected: {e}"),
    }

    Ok(())
}
