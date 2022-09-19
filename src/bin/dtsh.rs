use clap::{Parser, Subcommand};
use dt_tool::{DT_FATAL_PREFIX, DT_VERSION, DT_WARN_PREFIX};

const EXE_NAME: &str = "dtsh";

const CONVENTIONS: dt_tool::RunConventions = dt_tool::RunConventions {
    exe_name: EXE_NAME,
    exe_version: DT_VERSION,
    warn_prefix: DT_WARN_PREFIX,
    fatal_prefix: DT_FATAL_PREFIX,
};

pub fn main() {
    let args = DtShell::parse();

    let state = dt_tool::initial_state(args.no_stdlib, args.lib_list, &CONVENTIONS);

    match args.mode {
        Some(Mode::Interactive) | Some(Mode::RunStdin) | None => {
            dt_tool::run_prompt(state, &CONVENTIONS)
        }
        Some(Mode::Run { file }) => {
            let tokens = dt_tool::load_from_source_file(file);
            state.run_tokens(tokens);
        }
    }
}

#[derive(Parser)]
#[clap(name = EXE_NAME, version = DT_VERSION)]
/// dt shell. It's duck tape for your unix pipes
struct DtShell {
    #[clap(subcommand)]
    mode: Option<Mode>,

    #[clap(long)]
    /// Disable loading the dt standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,
}

#[derive(Subcommand)]
enum Mode {
    #[clap(visible_alias = "i")]
    /// Start an interactive session. (Default when no subcommand specified)
    Interactive,

    #[clap(visible_alias = "r")]
    /// Execute a file.
    Run { file: String },

    #[clap(name = "-")]
    /// Interpret code from standard input.
    RunStdin,
}
