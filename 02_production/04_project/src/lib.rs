// lib.rs
pub mod error;
pub mod files;

// re-export modules from the root
pub use crate::error::{Error, Result};
