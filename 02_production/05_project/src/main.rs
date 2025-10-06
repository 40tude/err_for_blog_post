// main.rs
// cargo run -p step_05

// ! In error.rs
// ! Remove Custom(String) variant from Error enum

use step_05::Result;
use step_05::files::listing;

fn main() -> Result<()> {
    match listing::list_files(".") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error: {e}"),
    }

    match listing::list_files("./02_production/05_project/empty") {
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
