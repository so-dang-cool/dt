use std::env;
use std::fs;
use std::path::Path;

use crate::rail_machine::Quote;
use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("cd", &["string"], &[], |quote| {
            let (path, quote) = quote.pop_string("cd");
            let path = Path::new(&path);
            env::set_current_dir(path).unwrap();
            quote
        }),
        RailDef::on_quote("ls", &[], &["quote"], |quote| {
            let path = env::current_dir().unwrap();
            let files = fs::read_dir(path).unwrap().filter(|dir| dir.is_ok()).fold(
                Quote::default(),
                |quote, dir| {
                    let dir = dir.unwrap().file_name().to_string_lossy().to_string();
                    quote.push_string(dir)
                },
            );
            quote.push_quote(files)
        }),
        RailDef::on_quote("pwd", &[], &["string"], |quote| {
            let path = env::current_dir().unwrap().to_string_lossy().to_string();
            quote.push_string(path)
        }),
        RailDef::on_quote("dir?", &["string"], &["bool"], |quote| {
            let (path, quote) = quote.pop_string("dir?");
            let path = Path::new(&path);
            quote.push_bool(path.is_dir())
        }),
        RailDef::on_quote("file?", &["string"], &["bool"], |quote| {
            let (path, quote) = quote.pop_string("file?");
            let path = Path::new(&path);
            quote.push_bool(path.is_file())
        }),
        RailDef::on_quote("mkdir", &["string"], &[], |quote| {
            let (path, quote) = quote.pop_string("mkdir");
            let path = Path::new(&path);
            fs::create_dir(path).unwrap();
            quote
        }),
        RailDef::on_quote("readf", &["string"], &["string"], |quote| {
            let (path, quote) = quote.pop_string("readf");
            let path = Path::new(&path);
            let contents = fs::read_to_string(path).unwrap();
            quote.push_string(contents)
        }),
        RailDef::on_quote("writef", &["string", "string"], &[], |quote| {
            let (path, quote) = quote.pop_string("writef");
            let (contents, quote) = quote.pop_string("writef");
            let path = Path::new(&path);
            fs::write(path, contents).unwrap();
            quote
        }),
    ]
}
