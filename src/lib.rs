use std::{
    env,
    path::{Path, PathBuf},
};

pub mod corelib;
pub mod prompt;
pub mod rail_machine;
pub mod tokens;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

pub fn rail_lib_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/rail/{}", home, RAIL_VERSION);
    Path::new(&path).to_owned()
}
