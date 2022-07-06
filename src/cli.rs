use crate::prompt::RailPrompt;
use crate::rail_machine::RailState;
use crate::tokens::{tokens_from_lib_list, tokens_from_rail_source};
use crate::RAIL_VERSION;
use clap::{Parser, Subcommand};

pub fn run() {
    let args = Cli::parse();

    let state = match args.no_stdlib {
        true => RailState::default(),
        false => {
            let tokens = tokens_from_lib_list("stdlib/all.txt");
            RailState::default().run_tokens(tokens)
        }
    };

    let state = match args.lib_list {
        Some(lib_list_file) => {
            let tokens = tokens_from_lib_list(&lib_list_file);
            state.run_tokens(tokens)
        }
        None => state,
    };

    match args.mode {
        Mode::Compile { output: _ } => unimplemented!("I don't know how to compile yet"),
        Mode::Interactive => RailPrompt::default().run(state),
        Mode::Run { file } => {
            let tokens = tokens_from_rail_source(file);
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
