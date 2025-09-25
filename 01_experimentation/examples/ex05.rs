// ex05.rs
// cargo run --example ex05

// ! custom error as formatted String

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    if files.is_empty() {
        // `?` is not the same thing as .into()
        //      `?` performs early return
        //      .into() just converts a value
        //
        // The `.into()` below works because std lib includes
        // impl<'a> From<&str> for Box<dyn Error + 'a>
        //      read : https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E
        //      The variants for String and + Send + Sync also exist
        //
        // When we write "Cannot list empty folder.".into();
        // It starts as a &'static str
        // Rust sees that the expected type is Box<dyn Error>
        // It found impl<'a> From<&str> for Box<dyn Error + 'a> in the std lib
        // In Rust if we have From<A> to B we get Into<B> for A for free
        // Here this means Into<Box<dyn Error> for &str exists
        // Then the &str is automatically converted to Box<dyn Error>
        return Err("Cannot list empty folder.".into());
    }
    Ok(files)
}
