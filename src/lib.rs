use std::{
    env, iter,
    path::{Path, PathBuf},
};

use rail_lang::{
    corelib::rail_builtin_dictionary,
    rail_machine::{self, RailDef, RailRunResult, RailState, RailType},
    tokens::Token,
};
use rail_lang::{loading, prompt::RailPrompt, SourceConventions};
pub use rail_lang::{log, RunConventions};

pub const DT_VERSION: &str = std::env!("CARGO_PKG_VERSION");
pub const DT_INFO_PREFIX: &str = "";
pub const DT_WARN_PREFIX: &str = "WARN: ";
pub const DT_ERR_PREFIX: &str = "ERR: ";
pub const DT_FATAL_PREFIX: &str = "RIP: ";

const DT_SOURCE_CONVENTIONS: SourceConventions = SourceConventions {
    lib_exts: &[".dt"],
    lib_list_exts: &[".txt"],
};

pub const fn dt_exe_conventions(exe_name: &str) -> RunConventions {
    RunConventions {
        exe_name,
        exe_version: DT_VERSION,
        info_prefix: DT_INFO_PREFIX,
        warn_prefix: DT_WARN_PREFIX,
        error_prefix: DT_ERR_PREFIX,
        fatal_prefix: DT_FATAL_PREFIX,
    }
}

// pub for dtup
pub fn dt_lib_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/dt/{}", home, DT_VERSION);
    Path::new(&path).to_owned()
}

fn stdlib_tokens(conv: &'static RunConventions) -> Vec<Token> {
    let path = dt_lib_path().join("dt-src/stdlib/all.txt");

    if path.is_file() {
        return loading::from_lib_list(path, &DT_SOURCE_CONVENTIONS);
    }

    log::error(conv, format!("Unable to load stdlib. Wanted to find it at {:?}\nDo you need to run 'dtup bootstrap'?",
    path));

    vec![]
}

pub fn initial_state(
    skip_stdlib: bool,
    lib_list: Option<String>,
    conv: &'static RunConventions,
) -> RailRunResult {
    let definitions = rail_builtin_dictionary();
    let definitions = definitions
        .values()
        .map(|def| def.to_owned().rename(namespace_most_rail_things))
        .chain(iter::once(RailDef::on_state(
            "version",
            "Produces the version of dt currently in use.",
            &[],
            &[RailType::String],
            |quote| Ok(quote.push_str(DT_VERSION)),
        )));

    let state = RailState::new_main(rail_machine::dictionary_of(definitions), conv);

    let state = match skip_stdlib {
        true => Ok(state),
        false => state.run_tokens(stdlib_tokens(conv)),
    };

    let state = log::error_coerce(state);

    match lib_list {
        Some(ll) => state.run_tokens(loading::from_lib_list(ll, &DT_SOURCE_CONVENTIONS)),
        None => Ok(state),
    }
}

pub fn namespace_most_rail_things(name: String) -> String {
    match name.as_str() {
        // These do not get namespaced at all, and we rely on Rail to be backward-compatible with these.
        "do!" | "doin!" | "alias" | "def!" | "each!" | "->" | "=>" => name,
        // These get namespaced. Some are re-exported in 'dt-src/stdlib/1_bootstrap_rail_defs.dt'
        // but that area is preserved in case Rail behavior changes.
        _ => String::from("rail/") + &name,
    }
}

pub fn load_from_source(source: String) -> Vec<Token> {
    loading::get_source_as_tokens(source)
}

pub fn load_from_source_file(file: String) -> Vec<Token> {
    loading::get_source_file_as_tokens(file)
}

pub fn run_prompt(state: RailState, conv: &'static RunConventions) -> RailRunResult {
    RailPrompt::new(conv).run(state)
}
