// error.rs
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

// Custom error type for the project
// This design allows us to handle both custom business logic errors and system-level I/O errors through the same error type
// With `thiserror`, we can attach per-variant messages and automatic conversions (`#[from]`) without writing the boilerplate manually
#[derive(Debug, Error)]
pub enum Error {
    // Handles application-specific errors with custom messages.
    // `Display` comes from the `#[error]` attribute.
    #[error("Custom error - {0}")]
    Custom(String),

    // Wraps standard I/O errors.
    // `#[from]` gives us `From<std::io::Error> for Error` automatically.
    // Without transparent => Error: I/O error: No such file or directory (os error 2)
    // With transparent    => Error: No such file or directory (os error 2)
    //
    #[error(transparent)]
    // #[error("**** I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    // A convenience constructor that accepts anything implementing `Display` (string literals, numbers...)
    // It converts `val` to a Error::Custom variant
    // This is more flexible than requiring a String directly
    // This allows us to write `Error::custom("foo")` instead of manually allocating a `String`.
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

// In Rust if the trait From<A> for B exists, then we get the trait Into<B> for A for free
// Here we define From<&str> for Error => Into<Error> for &str is available
// We can write : return Err("something went wrong".into()).
impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
