use std::env;

use crate::dt_machine::{self, Definition, DtType, DtValue};

use DtType::*;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        Definition::on_state("exec", &[String], &[Quote], |quote| {
            let (invocation, quote) = quote.pop_string("exec");
            let invocation = invocation.trim();
            let (exe, args) = invocation.split_once(' ').unwrap_or((invocation, ""));
            let args = args.split_ascii_whitespace().collect::<Vec<_>>();

            let res = std::process::Command::new(exe).args(args).output().unwrap();

            let mut result = dt_machine::new_stab();
            result.insert(
                "status".to_string(),
                DtValue::I64(res.status.code().unwrap_or(-1).into()),
            );
            result.insert(
                "stdout".to_string(),
                DtValue::String(
                    std::string::String::from_utf8(res.stdout)
                        .unwrap()
                        .trim_end()
                        .to_string(),
                ),
            );
            result.insert(
                "stderr".to_string(),
                DtValue::String(
                    std::string::String::from_utf8(res.stderr)
                        .unwrap()
                        .trim_end()
                        .to_string(),
                ),
            );

            quote.push_stab(result)
        }),
        Definition::on_state("env", &[], &[String], |quote| {
            let vars = env::vars().fold(dt_machine::new_stab(), |mut stab, (k, v)| {
                stab.insert(k, DtValue::String(v));
                stab
            });
            quote.push_stab(vars)
        }),
        Definition::on_state("envget", &[String], &[String], |quote| {
            let (key, quote) = quote.pop_string("envget");
            let var = env::var(key).unwrap_or_else(|_| "unset".to_string());
            quote.push_string(var)
        }),
        Definition::on_state("envset", &[String, String], &[], |quote| {
            let (var, quote) = quote.pop_string("envset");
            let (key, quote) = quote.pop_string("envset");
            env::set_var(key, var);
            quote
        }),
        Definition::on_state("stdin", &[], &[Quote], |quote| {
            let lines = std::io::stdin()
                .lines()
                .filter_map(|line| line.ok())
                .fold(quote.child(), |quote, line| quote.push_string(line));
            quote.push_quote(lines)
        }),
    ]
}
