// error.rs
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

// We define a custom error type for the project
// This design allows us to handle both custom business logic errors and system-level I/O errors through the same error type
// With derive_more we get automatic conversion from String and std::io::Error into our Error type
#[derive(Debug, From)]
pub enum Error {
    //
    #[from]
    Custom(String), // handles application-specific errors with custom messages

    #[from]
    Io(std::io::Error), // wraps standard I/O errors
}

impl Error {
    // The custom method provides a constructor that accepts anything implementing Display (string literals, numbers...)
    // It converts it to a Custom error variant.
    // This is more flexible than requiring a String directly.
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

// The manual impl below allows direct conversion from string slices to our error type
// It is "mandatory" because
// #[from]
//      Custom(&str),
// Is not easy to compile (lifetime issues etc.) and would be confusing
// In addition: In Rust if the trait From<A> for B exists, then we get the trait Into<B> for A for free
// Here we define From<&str> for Error => Into<Error> for &str is available
// We can write : return Err("something went wrong".into()).
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

// The Display implementation takes a shortcut by using debug formatting ({self:?}) instead of providing user-friendly messages.
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), core::fmt::Error> {
        write!(fmt, "🔎 {self:?}") // only debug print here
    }
}

// Implement the standard Error trait for integration with other error tooling.
// Error must implement Debug and Display (see above )
impl std::error::Error for Error {}
