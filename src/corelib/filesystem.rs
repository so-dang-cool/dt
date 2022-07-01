use std::env;
use std::fs;
use std::path::Path;

use crate::rail_machine::Quote;
use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack("cd", &["string"], &[], |stack| {
            let (path, stack) = stack.pop_string("cd");
            let path = Path::new(&path);
            env::set_current_dir(path).unwrap();
            stack
        }),
        RailDef::on_stack("ls", &[], &["quote"], |stack| {
            let path = env::current_dir().unwrap();
            let quote = fs::read_dir(path).unwrap().filter(|dir| dir.is_ok()).fold(
                Quote::default(),
                |quote, dir| {
                    let dir = dir.unwrap().file_name().to_string_lossy().to_string();
                    quote.push_string(dir)
                },
            );
            stack.push_quote(quote)
        }),
        RailDef::on_stack("pwd", &[], &["string"], |stack| {
            let path = env::current_dir().unwrap().to_string_lossy().to_string();
            stack.push_string(path)
        }),
        RailDef::on_stack("dir?", &["string"], &["bool"], |stack| {
            let (path, stack) = stack.pop_string("dir?");
            let path = Path::new(&path);
            stack.push_bool(path.is_dir())
        }),
        RailDef::on_stack("file?", &["string"], &["bool"], |stack| {
            let (path, stack) = stack.pop_string("file?");
            let path = Path::new(&path);
            stack.push_bool(path.is_file())
        }),
        RailDef::on_stack("mkdir", &["string"], &[], |stack| {
            let (path, stack) = stack.pop_string("mkdir");
            let path = Path::new(&path);
            fs::create_dir(path).unwrap();
            stack
        }),
        RailDef::on_stack("readf", &["string"], &["string"], |stack| {
            let (path, stack) = stack.pop_string("readf");
            let path = Path::new(&path);
            let contents = fs::read_to_string(path).unwrap();
            stack.push_string(contents)
        }),
        RailDef::on_stack("writef", &["string", "string"], &[], |stack| {
            let (path, stack) = stack.pop_string("writef");
            let (contents, stack) = stack.pop_string("writef");
            let path = Path::new(&path);
            fs::write(path, contents).unwrap();
            stack
        }),
    ]
}
