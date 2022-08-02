use clap::Parser;
use rail_lang::rail_machine::RailState;
use rail_lang::RAIL_VERSION;

pub fn main() {
    let args = RailCompiler::parse();

    let _state = RailState::new_with_libs(args.no_stdlib, args.lib_list);

    eprintln!("I'm not implemented yet.");
    std::process::exit(1);
}

#[derive(Parser)]
#[clap(name = "railc", version = RAIL_VERSION)]
/// Rail Compiler. A straightforward programming language
struct RailCompiler {
    #[clap(long)]
    /// Disable loading the Rail standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,
}
