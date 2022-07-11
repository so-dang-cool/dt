use std::env;

use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("exec", &["string"], &["quote"], |quote| {
            let (invocation, quote) = quote.pop_string("exec");
            let invocation = invocation.trim();
            let (exe, args) = invocation.split_once(' ').unwrap_or((invocation, ""));
            let args = args.split_ascii_whitespace().collect::<Vec<_>>();

            let res = std::process::Command::new(exe).args(args).output().unwrap();

            let result = Quote::default()
                .push_i64(res.status.code().unwrap_or(-1).into())
                .push_str(String::from_utf8(res.stdout).unwrap().trim_end())
                .push_str(String::from_utf8(res.stderr).unwrap().trim_end());

            quote.push_quote(result)
        }),
        RailDef::on_quote("env", &[], &["string"], |quote| {
            let vars = env::vars()
                .map(|(k, v)| Quote::default().push_string(k).push_string(v))
                .fold(Quote::default(), |q, kv| q.push_quote(kv));
            quote.push_quote(vars)
        }),
        RailDef::on_quote("envget", &["string"], &["string"], |quote| {
            let (key, quote) = quote.pop_string("envget");
            let var = env::var(key).unwrap_or_else(|_| "unset".to_string());
            quote.push_string(var)
        }),
        RailDef::on_quote("envset", &["string", "string"], &[], |quote| {
            let (var, quote) = quote.pop_string("envset");
            let (key, quote) = quote.pop_string("envset");
            env::set_var(key, var);
            quote
        }),
        RailDef::on_quote("stdin", &[], &["quote"], |quote| {
            let lines = std::io::stdin()
                .lines()
                .filter_map(|line| line.ok())
                .fold(Quote::default(), |quote, line| quote.push_string(line));
            quote.push_quote(lines)
        }),
    ]
}
