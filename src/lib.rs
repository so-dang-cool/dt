pub mod cli;
pub mod corelib;
pub mod prompt;
pub mod rail_machine;
pub mod tokens;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");
