use clap::{Parser, Subcommand};
use dt_tool::dt_machine::DtState;
use dt_tool::prompt::DtPrompt;
use dt_tool::{loading, DT_VERSION};

pub fn main() {
    let args = DtShell::parse();

    let state = DtState::new_with_libs(args.no_stdlib, args.lib_list);

    match args.mode {
        Some(Mode::Interactive) | Some(Mode::RunStdin) | None => DtPrompt::default().run(state),
        Some(Mode::Run { file }) => {
            let tokens = loading::from_dt_source_file(file);
            state.run_tokens(tokens);
        }
    }
}

#[derive(Parser)]
#[clap(name = "dtsh", version = DT_VERSION)]
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
