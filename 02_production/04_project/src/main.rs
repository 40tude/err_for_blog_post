// main.rs
// cargo run -p step_02

// ! Add derive_more

use step_04::Result; // uses the re-export from the lib
use step_04::files::listing;

fn main() -> Result<()> {
    match listing::list_files(".") {
        Ok(files) => println!("Files found: {files:#?}"),
        Err(e) => println!("Error: {e}"),
    }

    match listing::list_files("./02_production/03_project/empty") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error detected: {e}"),
    }

    match listing::list_files("./non_existent_folder") {
        Ok(files) => println!("Files found   : {files:#?}"),
        Err(e) => println!("Error detected: {e}"),
    }

    Ok(())
}
