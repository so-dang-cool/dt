use clap::Parser;
use rail_lang::rail_machine;
use rail_lang::{loading, RAIL_VERSION};

pub fn main() {
    let args = RailEvaluator::parse();

    let state = rail_machine::state_with_libs(args.no_stdlib, args.lib_list);

    let tokens = loading::from_rail_source(args.rail_code.join(" "));
    state.run_tokens(tokens);
}

#[derive(Parser)]
#[clap(name = "rail", version = RAIL_VERSION)]
/// Rail Evaluator. A straightforward programming language
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
