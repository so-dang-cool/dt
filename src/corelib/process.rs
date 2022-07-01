use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_stack(
        "exec",
        &["string"],
        &["quote"],
        |stack| {
            let (invocation, stack) = stack.pop_string("exec");
            let invocation = invocation.trim();
            let (exe, args) = invocation.split_once(' ').unwrap_or((invocation, ""));
            let args = args.split_ascii_whitespace().collect::<Vec<_>>();

            let res = std::process::Command::new(exe).args(args).output().unwrap();

            let quote = Quote::default()
                .push_i64(res.status.code().unwrap_or(-1).into())
                .push_str(String::from_utf8(res.stdout).unwrap().trim_end())
                .push_str(String::from_utf8(res.stderr).unwrap().trim_end());

            stack.push_quote(quote)
        },
    )]
}
