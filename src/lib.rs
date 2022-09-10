use std::{
    env,
    path::{Path, PathBuf},
};

pub mod corelib;
pub mod dt_machine;
pub mod loading;
pub mod prompt;
pub mod tokens;

pub const DT_VERSION: &str = std::env!("CARGO_PKG_VERSION");

pub fn dt_lib_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/dt/{}", home, DT_VERSION);
    Path::new(&path).to_owned()
}
