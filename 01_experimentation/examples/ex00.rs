// ex00.rs
// cargo run --example ex00

// ! type alias first use

// Type aliases let us write Result<T> instead of std::result::Result<T, Box<dyn Error>>.
// The Box<dyn Error> means “any error type that implements std::error::Error, packaged behind a trait object.”
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Since we want to use the same code from experimentation to production we keep Error and Result separated
// Here we want to keep the Error type alias because in production, Error may evolve in a custom error type based on a enum
// Result is tighten to current version of Error. In production Error may change but Result will not be impacted
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(()) // we must return a Result whose value here is Ok(())
}
