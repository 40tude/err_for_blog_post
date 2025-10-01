use derive_more::From;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {
    // -- listing
    // #[display()...)]  // ! Make a test with a CLI
    CantListEmptyFolder,

    #[from]
    Io(std::io::Error),
}

// Implements how our Error is printed with {}.
// Manadatory if we want our Error to used as a standard error (see next impl)
// It delegates to the Debug representation ({:?}), which is fine since it is not for user-facing messages
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}") // only debug print here
    }
}

// Marks our Error type as a standard error that can participate in error chains (as `?` for example)
impl std::error::Error for Error {}
