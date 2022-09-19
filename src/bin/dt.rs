use clap::Parser;
use dt_tool::{DT_FATAL_PREFIX, DT_VERSION, DT_WARN_PREFIX};

const EXE_NAME: &str = "dt";

const CONVENTIONS: dt_tool::RunConventions = dt_tool::RunConventions {
    exe_name: EXE_NAME,
    exe_version: DT_VERSION,
    warn_prefix: DT_WARN_PREFIX,
    fatal_prefix: DT_FATAL_PREFIX,
};

pub fn main() {
    let args = DtEvaluator::parse();

    let state = dt_tool::initial_state(args.no_stdlib, args.lib_list, &CONVENTIONS);

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

    let tokens = dt_tool::load_from_source(args.dt_code.join(" "));

    state.run_tokens(tokens);
}

#[derive(Parser)]
#[clap(name = EXE_NAME, version = DT_VERSION)]
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
