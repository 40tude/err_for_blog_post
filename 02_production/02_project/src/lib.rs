// lib.rs
pub mod error;
pub mod files;

// re-export lib from crate root
pub use self::error::{Error, Result};
