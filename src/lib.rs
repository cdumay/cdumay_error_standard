//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_standard on crates.io](https://img.shields.io/crates/v/cdumay_error_standard)](https://crates.io/crates/cdumay_error_standard)
//! [![cdumay_error_standard on docs.rs](https://docs.rs/cdumay_error_standard/badge.svg)](https://docs.rs/cdumay_error_standard)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_standard)
//!
//! A collection of standard error types and error kinds commonly used in Rust applications.
//! This crate provides predefined error types and kinds using the `cdumay_error` framework.
//! 
//! ## Features
//!
//! - Common error types for IO operations and unexpected errors
//! - Easy integration with the `cdumay_error` framework
//! 
//! ## Examples
//! 
//! ```rust
//! use cdumay_error_standard::{FileNotExists, Unexpected};
//! use std::path::Path;
//! 
//! // Creating a FileNotExists error
//! fn check_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//!     if !path.exists() {
//!         return Err(FileNotExists::new().set_message(format!(
//!             "File {} does not exist",
//!             path.display()
//!         )).into());
//!     }
//!     Ok(())
//! }
//! 
//! // Using Unexpected error for runtime errors
//! fn divide(a: i32, b: i32) -> Result<i32, Box<dyn std::error::Error>> {
//!     if b == 0 {
//!         return Err(Unexpected::new().set_message("Division by zero".into()).into());
//!     }
//!     Ok(a / b)
//! }
//! ```
//! 
//! ## Error Handling
//! 
//! All errors implement the `AsError` trait from `cdumay_error`, providing consistent
//! error handling across your application:
//! 
//! ```rust
//! use cdumay_error_standard::{FileRead};
//! use cdumay_error::AsError;
//! 
//! fn read_content() -> Result<String, Box<dyn std::error::Error>> {
//!     let err = FileRead::new().set_message("Failed to read config file".into());
//!     
//!     // Access error properties
//!     println!("Error code: {}", FileRead::kind.code());
//!     println!("Message: {}", err.message());
//!     
//!     Err(err.into())
//! }
//! ```

use cdumay_error::{define_errors, define_kinds, AsError};

define_kinds! {
    UnknownError = ("Err-00001", 500, "Unexpected error"),
    IoError = ("Err-00002", 400, "IO error"),
    ValidationError = ("Err-00003", 400, "Validation error")
}
define_errors! {
    Unexpected = UnknownError,
    FileRead = IoError,
    FileNotExists = IoError,
    DeserializationError = ValidationError,
    SerializationError = ValidationError
}