use std::{fmt::Debug, fs, path::Path};

use crate::prompt::RailPrompt;
use crate::rail_machine::RailState;
use crate::tokens::tokenize;
use crate::RAIL_VERSION;
use clap::{Parser, Subcommand};

pub fn run() {
    let args = Cli::parse();

    let state = match args.no_stdlib {
        true => RailState::default(),
        false => {
            let tokens = load_rail_files_to_tokens("stdlib");
            RailState::default().run_tokens(tokens)
        }
    };

    let state = match args.lib_list {
        Some(lib_list_file) => {
            let tokens = load_lib_list_files_to_tokens(&lib_list_file);
            state.run_tokens(tokens)
        }
        None => state,
    };

    match args.mode {
        Mode::Compile { output: _ } => unimplemented!("I don't know how to compile yet"),
        Mode::Interactive => RailPrompt::default().run(state),
        Mode::Run { file } => {
            let tokens = load_rail_file_to_tokens(file);
            state.run_tokens(tokens);
        }
        Mode::RunStdin => unimplemented!("I don't know how to run stdin yet"),
    }
}

#[derive(Parser)]
#[clap(name = "rail", version = RAIL_VERSION)]
/// An organizing tool for terminal lovers who hate organizing
struct Cli {
    #[clap(subcommand)]
    mode: Mode,

    #[clap(long)]
    /// Disable loading the Rail standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,
}

#[derive(Subcommand)]
enum Mode {
    #[clap(visible_alias = "i")]
    /// Start an interactive session.
    Interactive,

    #[clap(visible_alias = "c")]
    /// Compile to native.
    Compile {
        #[clap(short = 'o')]
        output: Option<String>,
    },

    #[clap(visible_alias = "r")]
    /// Execute a file.
    Run { file: String },

    #[clap(name = "-")]
    /// Read from standard input.
    RunStdin,
}

fn load_rail_source_to_tokens(contents: String) -> Vec<String> {
    contents.split('\n').flat_map(tokenize).collect::<Vec<_>>()
}

fn load_rail_file_to_tokens<P>(path: P) -> Vec<String>
where
    P: AsRef<Path> + Debug,
{
    let error_msg = format!("Error reading file {:?}", path);
    let contents = fs::read_to_string(path).expect(&error_msg);
    load_rail_source_to_tokens(contents)
}

// TODO: Update this to only be lib lists, no nondeterministic order dir globbing
fn load_rail_files_to_tokens(path: &str) -> Vec<String> {
    let stdlibs = fs::read_dir(path).unwrap_or_else(|_| {
        panic!(
            "Did not find {} in current directory {:?}",
            path,
            std::env::current_dir()
        )
    });

    stdlibs
        .filter_map(|f| f.ok())
        .map(|entry| entry.path())
        .flat_map(load_rail_file_to_tokens)
        .collect::<Vec<_>>()
}

fn load_lib_list_files_to_tokens(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to load library list file {}", path))
        .split('\n')
        .filter(|s| !s.is_empty())
        .flat_map(|path| {
            if path.ends_with(".rail") {
                load_rail_file_to_tokens(path)
            } else {
                load_rail_files_to_tokens(path)
            }
        })
        .collect::<Vec<_>>()
}
