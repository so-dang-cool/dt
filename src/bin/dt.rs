use clap::Parser;
use dt_tool::dt_machine::DtState;
use dt_tool::{loading, DT_VERSION};

pub fn main() {
    let args = DtEvaluator::parse();

    let state = DtState::new_with_libs(args.no_stdlib, args.lib_list);

    // Consume stdin by default when stdin is not a TTY (e.g. in a unix pipe)
    let state = if atty::isnt(atty::Stream::Stdin) {
        state
            .run_term("stdin")
            .run_term("quote-all")
            .run_term("prune")
            .run_term("...")
    } else {
        state
    };

    let tokens = loading::from_dt_source(args.dt_code.join(" "));

    state.run_tokens(tokens);
}

#[derive(Parser)]
#[clap(name = "dt", version = DT_VERSION)]
/// dt evaluator. It's duck tape for your unix pipes
struct DtEvaluator {
    #[clap(long)]
    /// Disable loading the dt standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,

    /// Code to evaluate
    dt_code: Vec<String>,
}
