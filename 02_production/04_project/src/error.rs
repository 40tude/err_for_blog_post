// error.rs.03

use derive_more::From;
// use derive_more::{From, Display}; // For CLI for example
pub type Result<T> = std::result::Result<T, Error>;

// For CLI add Display
// For Web we could add Serialize
#[derive(Debug, From)]
pub enum Error {
    // -- my_lib
    // #[display()...)]  // ! Make a test with a CLI
    CantListEmptyFolder, // "Error: CantListEmptyFolder" is displayed

    // -- Externals
    #[from]
    Io(std::io::Error),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}") // only debug print here
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
