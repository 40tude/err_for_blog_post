// error.rs
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

// Custom error type for the project
// This design allows us to handle both custom business logic errors and system-level I/O errors through the same error type
// With `thiserror`, we can attach per-variant messages and automatic conversions (`#[from]`) without writing the boilerplate manually
#[derive(Debug, Error)]
pub enum Error {
    // The folder exists but is empty
    // `Display` comes from the `#[error]` attribute.
    #[error("⛔ Cannot list an empty folder")]
    CantListEmptyFolder,

    // Wraps I/O errors (e.g. file not found, permission denied, etc.)
    // `#[from]` gives us `From<std::io::Error> for Error` automatically.
    #[error(transparent)]
    // #[error("⛔ I/O error: {0}")]
    Io(#[from] std::io::Error),
}
