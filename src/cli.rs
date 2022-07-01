use clap::{Parser, Subcommand};
use crate::prompt::{operate_term, RailPrompt};
use crate::rail_machine::RailState;
use crate::tokens::tokenize;
use crate::RAIL_VERSION;

pub fn run() {
    let args = Cli::parse();

    let state = if args.no_stdlib {
        RailState::default()
    } else {
        load_rail_stdlib()
    };

    if let Some(_lib_list) = args.lib_list {
        unimplemented!("I don't know how to load library lists yet")
    }

    match args.mode {
        Mode::Compile { output: _ } => unimplemented!("I don't know how to compile yet"),
        Mode::Interactive => RailPrompt::default().run(state),
        Mode::Run { file: _ } => unimplemented!("I don't know how to run files yet"),
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
    /// Execute a file with a defined "main" command.
    Run { file: String },

    #[clap(name = "-")]
    /// Read from standard input.
    RunStdin,
}

fn load_rail_stdlib() -> RailState {
    let stdlibs = std::fs::read_dir("stdlib").expect("Did not find stdlib in current directory!");

    stdlibs
        .filter_map(|f| f.ok())
        .map(|entry| entry.path())
        .map(|path| std::fs::read_to_string(path).expect("Error reading file"))
        .flat_map(|contents| tokenize(&contents))
        .fold(RailState::default(), operate_term)
}
