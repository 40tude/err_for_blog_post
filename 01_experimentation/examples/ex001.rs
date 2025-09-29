// ex001.rs
// cargo run -p experimentation --example ex001

// ! type alias better use

// Type aliases let us write Result<T> instead of std::result::Result<T, Box<dyn Error>>.
// The Box<dyn Error> means “any error type that implements std::error::Error, packaged behind a trait object.”

// We could write:
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// However, since we want to use the same code from experimentation to production we keep Error and Result separated
// Here we want to keep the Error type alias because in production, Error may evolve in a custom error type based on a `enum`
// Result is tighten to current version of Error.
// In production Error will change but Result will not be impacted and this is exactly what we want
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(()) // we must return a Result whose value here is Ok(())
}
