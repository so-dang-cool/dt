use std::env;
use std::fs;
use std::path::Path;

use crate::dt_machine::{Definition, DtType};

use DtType::*;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        Definition::on_state("cd", &[String], &[], |quote| {
            let (path, quote) = quote.pop_string("cd");
            let path = Path::new(&path);
            env::set_current_dir(path).unwrap();
            quote
        }),
        Definition::on_state("ls", &[], &[Quote], |state| {
            let path = env::current_dir().unwrap();

            let files = fs::read_dir(path).unwrap().filter(|dir| dir.is_ok()).fold(
                state.child(),
                |quote, dir| {
                    let dir = dir.unwrap().file_name().to_string_lossy().to_string();
                    quote.push_string(dir)
                },
            );

            state.push_quote(files)
        }),
        Definition::on_state("pwd", &[], &[String], |quote| {
            let path = env::current_dir().unwrap().to_string_lossy().to_string();
            quote.push_string(path)
        }),
        Definition::on_state("dir?", &[String], &[Boolean], |quote| {
            let (path, quote) = quote.pop_string("dir?");
            let path = Path::new(&path);
            quote.push_bool(path.is_dir())
        }),
        Definition::on_state("file?", &[String], &[Boolean], |quote| {
            let (path, quote) = quote.pop_string("file?");
            let path = Path::new(&path);
            quote.push_bool(path.is_file())
        }),
        Definition::on_state("mkdir", &[String], &[], |quote| {
            let (path, quote) = quote.pop_string("mkdir");
            let path = Path::new(&path);
            fs::create_dir(path).unwrap();
            quote
        }),
        Definition::on_state("readf", &[String], &[String], |quote| {
            let (path, quote) = quote.pop_string("readf");
            let path = Path::new(&path);
            let contents = fs::read_to_string(path).unwrap();
            quote.push_string(contents)
        }),
        Definition::on_state("writef", &[String, String], &[], |quote| {
            let (path, quote) = quote.pop_string("writef");
            let (contents, quote) = quote.pop_string("writef");
            let path = Path::new(&path);
            fs::write(path, contents).unwrap();
            quote
        }),
    ]
}
