use clap::{Parser, Subcommand};
use rail_lang::{RAIL_VERSION, rail_lib_path};

pub fn main() {
    let args = RailUpdater::parse();

    match args.mode {
        Mode::Bootstrap => {
            let path = rail_lib_path();
            std::fs::create_dir_all(path.clone()).unwrap_or_else(|e| panic!("Couldn't create {:?}. {}", path, e));
            std::env::set_current_dir(path.clone()).unwrap_or_else(|e| panic!("Couldn't access {:?}. {}", path, e));
            let version_tag = format!("v{}", RAIL_VERSION);
            let clone_result = std::process::Command::new("git").args(["clone", "--single-branch", "--branch", &version_tag, "https://github.com/hiljusti/rail", &path.to_string_lossy()]).output().expect("Error running git clone");
            if !clone_result.status.success() {
                eprintln!("{}", String::from_utf8(clone_result.stderr).unwrap())
            }
        }
    }
}

#[derive(Parser)]
#[clap(name = "railup", version = RAIL_VERSION)]
/// Rail Updater. Provides updates for the Rail programming language
struct RailUpdater {
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    #[clap(visible_alias = "b")]
    /// Fetch the Rail std library and extras
    Bootstrap,
}
