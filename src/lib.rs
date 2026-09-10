//! FalconCore language core.
//!
//! The public modules intentionally expose the compiler pipeline so tooling can
//! embed FalconCore without shelling out to the CLI.

pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod vm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
