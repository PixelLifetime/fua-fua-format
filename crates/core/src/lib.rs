pub mod lexer;
pub mod syntax;
pub mod parser;
pub mod config;
pub mod formatter;

// Workaround for num-derive generating `core::option::Option` when the crate itself is named `core`
pub use std::option;