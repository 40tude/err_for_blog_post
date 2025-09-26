// ex001.rs
// cargo run -p experimentation --example ex001

// ! call list_files() from main()

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// main() returns Result<()>
// If anything inside the body of `main()` returns an Err, the `?` operator early-returns that error all the way out of main()
// The runtime then print the error
fn main() -> Result<()> {
    // Remainder about the ? operator:
    // On a value of type Result<T, E>, x? does two things:
    // 1. If x is Ok(v),  it evaluates to v
    // 2. If x is Err(e), it returns early from the current function with Err(From::from(e)).
    // That last bit means it converts the error `e` to the main() function’s error type (here, Box<dyn Error>) using From
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)? // Gives an iterator of io::Result<DirEntry>. `?` works here
        //
        // .filter_map(), .filter() and .collect() operate on an Iterator<Item = DirEntry> once the Result has been unwrapped by `?`
        // These iterator methods do not return a Result - they cannot fail in a way that would require error propagation
        // They simply transform the data from one form to another => No `?` at the end
        //
        .filter_map(|re| re.ok()) // silently drops entries that errored
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false)) // ask the filesystem whether the entry is a file; if that check errors, treated as false.
        .filter_map(|e| e.file_name().into_string().ok()) // keeps names that are valid UTF-8 (others are dropped)
        .collect();
    Ok(files)
}
