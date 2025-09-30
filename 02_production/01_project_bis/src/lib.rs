// lib.rs
pub mod error;
pub mod files;

pub use self::error::{Error, Result}; // re-export to crate root
