use crate::rail_machine::{RailOp, Stack};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![RailOp::on_stack("execute", &["s"], &["quot"], |stack| {
        let (invocation, stack) = stack.pop_string("execute ");
        let invocation = invocation.trim();
        let (exe, args) = invocation.split_once(' ').unwrap_or((invocation, ""));
        let args = args.split_ascii_whitespace().collect::<Vec<_>>();

        let res = std::process::Command::new(exe).args(args).output().unwrap();

        let quot = Stack::new()
            .push_i64(res.status.code().unwrap_or(-1).into())
            .push_str(String::from_utf8(res.stdout).unwrap().trim_end())
            .push_str(String::from_utf8(res.stderr).unwrap().trim_end());

        stack.push_quotation(quot)
    })]
}
