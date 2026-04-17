pub mod lexer;
pub mod syntax;
pub mod parser;
pub mod config;
pub mod formatter;
pub mod plugins;

// Workaround for num-derive generating `core::option::Option` when the crate itself is named `core`
pub use std::option;