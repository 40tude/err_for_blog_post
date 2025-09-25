// ex02.rs
// cargo run --example ex02

// ! custom error as static &str

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files("")?;
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path) // no `?` here
        //
        // If the Result is an Ok(value), the ? operator unwraps `value` and continues execution
        // If the Result is Err(e), .map_err() applies the closure to e and returns Err(closure(e))
        // Here the closure ignore the actual io::Error (|_| discards it) and replace it with a static string slice "Error while reading dir."
        // The ? operator immediately returns that error from the current function.
        //
        // The return type is Result<Vec<String>, Box<dyn std::error::Error>>
        // So when the Err(&str) need to be bubbled up, Rust automatically applies From<&str> for Box<dyn Error> which is implemented by boxing the &str as a string error.
        // That’s why we can return a bare "Error while reading dir." and it gets "promoted" into a proper Box<dyn Error>.
        //
        // ! The promotion from &str to Box<dyn std::error::Error> works because stdlib includes
        // impl<'a> From<&str> for Box<dyn Error + 'a>
        //      Read : https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E
        //      The variants for String and + Send + Sync also exist
        //
        .map_err(|_| "❗Error while reading dir.")? // but `?` is here
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
