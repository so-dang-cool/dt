use std::{fmt::Debug, fs, path::Path};

use regex::Regex;

use crate::{rail_lib_path, rail_machine};

fn tokenize(line: &str) -> Vec<String> {
    // TODO: Validate that a line does not contain unterminated strings.
    // TODO: Allow for string escapes for quotes, newlines, etc
    let re: Regex = Regex::new(r#"(".*?"|\S*)"#).unwrap();
    let line = line.replace('\n', " ");
    re.captures_iter(&line)
        .flat_map(|cap| cap.iter().take(1).collect::<Vec<_>>())
        .filter_map(|res| res.map(|mat| mat.as_str()))
        .take_while(|s| !s.starts_with('#'))
        .filter(|s| !s.is_empty())
        .map(|s| s.replace("\\n", "\n"))
        .collect()
}

pub fn from_rail_source(source: String) -> Vec<String> {
    source.split('\n').flat_map(tokenize).collect()
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

    let message = format!("Unable to load stdlib. Wanted to find it at {:?}", path);
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

#[test]
pub fn token_test() {
    let actual = "1 1 +";
    let expected = vec!["1", "1", "+"];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_2() {
    let actual = "\"hello\" \"there\"";
    let expected = vec!["\"hello\"", "\"there\""];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_3() {
    let actual = "\"hello there\"";
    let expected = vec!["\"hello there\""];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_4() {
    let actual = "\" hello there \"";
    let expected = vec!["\" hello there \""];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_5() {
    let actual = "1 2 \" hello three \" 4 5";
    let expected = vec!["1", "2", "\" hello three \"", "4", "5"];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_6() {
    let actual = "1 2 \"a # in a string is fine\" #but at the end is ignored";
    let expected = vec!["1", "2", "\"a # in a string is fine\""];

    assert_eq!(expected, tokenize(actual));
}

#[test]
pub fn token_test_7() {
    let actual = "1 1 [ + ] call .s";
    let expected = vec!["1", "1", "[", "+", "]", "call", ".s"];

    assert_eq!(expected, tokenize(actual));
}
