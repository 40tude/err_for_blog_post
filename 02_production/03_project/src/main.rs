// main.rs
// cargo run -p step_02

// ! Add derive_more 14:18

use step_03::Result; // uses the re-export from the lib
use step_03::files::listing;

fn main() -> Result<()> {
    let files = listing::list_files("./02_production/02_project/empty")?;
    println!("{files:#?}");
    Ok(())
}
