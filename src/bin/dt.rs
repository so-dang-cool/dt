use clap::Parser;
use dt_tool::{dt_exe_conventions, log, RunConventions, DT_VERSION};
use rail_lang::tokens::Token;

const EXE_NAME: &str = "dt";
const CONV: RunConventions = dt_exe_conventions(EXE_NAME);

pub fn main() {
    let args = DtEvaluator::parse();

    let state = dt_tool::initial_state(args.no_stdlib, args.lib_list, &CONV);

    // Consume stdin by default when stdin is not a TTY (e.g. in a unix pipe)
    let state = if atty::isnt(atty::Stream::Stdin) {
        ["stdin", "quote-all", "prune", "..."]
            .into_iter()
            .map(|s| Token::from(s.to_string()))
            .fold(state, |res, tok| match res {
                Ok(state) => state.run_token(tok),
                Err((state, err)) => {
                    log::error(&CONV, format!("{:?}", err));
                    Ok(state)
                }
            })
    } else {
        state
    };

    let state = log::error_coerce(state);

    let tokens = dt_tool::load_from_source(args.dt_code.join(" "));

    if let Err((state, err)) = state.run_tokens(tokens) {
        let stack_dump = match state.len() {
            0 => String::from(""),
            _ => format!("\nStack dump: {}", state.stack),
        };
        rail_lang::log::error(&CONV, format!("Ended with error: {:?}{}", err, stack_dump));
    };
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
