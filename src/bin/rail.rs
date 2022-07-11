use clap::Parser;
use rail_lang::rail_machine::RailState;
use rail_lang::{tokens, RAIL_VERSION};

pub fn main() {
    let args = RailEvaluator::parse();

    let state = RailState::default();

    let state = match args.no_stdlib {
        true => RailState::default(),
        false => {
            let tokens = tokens::from_lib_list("rail-src/stdlib/all.txt");
            state.run_tokens(tokens)
        }
    };

    let state = match args.lib_list {
        Some(lib_list_file) => {
            let tokens = tokens::from_lib_list(&lib_list_file);
            state.run_tokens(tokens)
        }
        None => state,
    };

    let tokens = tokens::from_rail_source(args.rail_code.join(" "));
    state.run_tokens(tokens);
}

#[derive(Parser)]
#[clap(name = "rail", version = RAIL_VERSION)]
/// A straightforward programming language
struct RailEvaluator {
    #[clap(long)]
    /// Disable loading the Rail standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,

    /// Code to evaluate
    rail_code: Vec<String>,
}
