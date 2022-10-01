use std::{
    env,
    path::{Path, PathBuf},
};

pub use rail_lang::RunConventions;
use rail_lang::{
    corelib::rail_builtin_dictionary,
    rail_machine::{self, RailState},
};
use rail_lang::{loading, prompt::RailPrompt, SourceConventions};

pub const DT_VERSION: &str = std::env!("CARGO_PKG_VERSION");
pub const DT_WARN_PREFIX: &str = "WARN";
pub const DT_FATAL_PREFIX: &str = "RIP";

const DT_SOURCE_CONVENTIONS: SourceConventions = SourceConventions {
    lib_exts: &[".dt"],
    lib_list_exts: &[".txt"],
};

// pub for dtup
pub fn dt_lib_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/dt/{}", home, DT_VERSION);
    Path::new(&path).to_owned()
}

fn stdlib_tokens(conv: &'static RunConventions) -> Vec<String> {
    let path = dt_lib_path().join("dt-src/stdlib/all.txt");

    if path.is_file() {
        return loading::from_lib_list(path, &DT_SOURCE_CONVENTIONS);
    }

    let message = format!(
        "Unable to load stdlib. Wanted to find it at {:?}\nDo you need to run 'dtup bootstrap'?",
        path
    );
    rail_machine::log_warn(conv, message);

    vec![]
}

pub fn initial_state(
    skip_stdlib: bool,
    lib_list: Option<String>,
    conv: &'static RunConventions,
) -> RailState {
    let definitions = rail_builtin_dictionary();
    let definitions = definitions
        .values()
        .map(|def| def.to_owned().rename(namespace_most_rail_things));

    let state = RailState::new_main(rail_machine::dictionary_of(definitions), conv);

    let state = match skip_stdlib {
        true => state,
        false => state.run_tokens(stdlib_tokens(conv)),
    };

    match lib_list {
        Some(ll) => state.run_tokens(loading::from_lib_list(ll, &DT_SOURCE_CONVENTIONS)),
        None => state,
    }
}

pub fn namespace_most_rail_things(name: String) -> String {
    match name.as_str() {
        "do!" | "doin!" | "def!" | "each!" => name,
        _ => String::from("rail/") + &name,
    }
}

pub fn load_from_source(source: String) -> Vec<String> {
    loading::get_source_as_tokens(source)
}

pub fn load_from_source_file(file: String) -> Vec<String> {
    loading::get_source_file_as_tokens(file)
}

pub fn run_prompt(state: RailState, conv: &'static RunConventions) {
    RailPrompt::new(conv).run(state)
}
