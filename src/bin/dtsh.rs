use clap::{Parser, Subcommand};
use dt_tool::{dt_exe_conventions, RunConventions, DT_VERSION};

const EXE_NAME: &str = "dtsh";
const CONV: RunConventions = dt_exe_conventions(EXE_NAME);

pub fn main() {
    let args = DtShell::parse();

    let result =
        dt_tool::initial_state(args.no_stdlib, args.lib_list, &CONV).map(|state| match args.mode {
            Some(Mode::Interactive) | Some(Mode::RunStdin) | None => {
                dt_tool::run_prompt(state, &CONV)
            }
            Some(Mode::Run { file }) => {
                let tokens = dt_tool::load_from_source_file(file);
                state.run_tokens(tokens)
            }
        });

    if let Err((state, err)) = result {
        let stack_dump = match state.len() {
            0 => String::from(""),
            _ => format!("\nStack dump: {}", state.stack),
        };
        rail_lang::log::error(&CONV, format!("Ended with error: {:?}{}", err, stack_dump));
    };
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
