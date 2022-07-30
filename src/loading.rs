use std::{fmt::Debug, fs, path::Path};

use crate::tokens;
use crate::{rail_lib_path, rail_machine};

pub fn from_rail_source(source: String) -> Vec<String> {
    source.split('\n').flat_map(tokens::tokenize).collect()
}

pub fn from_rail_source_file<P>(path: P) -> Vec<String>
where
    P: AsRef<Path> + Debug,
{
    let error_msg = format!("Error reading file {:?}", path);
    let source = fs::read_to_string(path).expect(&error_msg);

    from_rail_source(source)
}

pub fn from_stdlib() -> Vec<String> {
    let path = rail_lib_path().join("rail-src/stdlib/all.txt");

    if path.is_file() {
        return from_lib_list(path);
    }

    let message = format!(
        "Unable to load stdlib. Wanted to find it at {:?}\nDo you need to run 'railup bootstrap'?",
        path
    );
    rail_machine::log_warn(message);

    vec![]
}

pub fn from_lib_list<P>(path: P) -> Vec<String>
where
    P: AsRef<Path> + Debug,
{
    let path: &Path = path.as_ref();

    let base_dir = path.parent().unwrap();

    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to load library list file {:?}", path))
        .split('\n')
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .map(|filepath| base_dir.join(filepath).to_string_lossy().to_string())
        .map(|file| {
            if file.ends_with(".rail") {
                Some(from_rail_source_file(file))
            } else if file.ends_with(".txt") {
                Some(from_lib_list(file))
            } else {
                None
            }
        })
        .filter(|list| list.is_some())
        .flat_map(|list| list.unwrap())
        .collect::<Vec<_>>()
}
