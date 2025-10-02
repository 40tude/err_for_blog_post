// lib.rs
pub mod error;
pub mod files;

// re-export modules from the root
// pub use self::error::{Error, Result};
pub use crate::error::{Error, Result};
