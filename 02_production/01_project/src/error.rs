// error.rs.03

use derive_more::From;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- fs
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
