// ex000.rs
// cargo run -p experimentation --example ex000

// ! type alias first use

// Type aliases let us write Result<T> instead of std::result::Result<T, Box<dyn Error>>.
// The Box<dyn Error> means “any error type that implements std::error::Error, packaged behind a trait object.”

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(()) // we must return a Result whose value here is Ok(())
}
