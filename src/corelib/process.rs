use crate::RailOp;
use crate::Stack;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![RailOp::new("execute", &["s"], &["quot"], |state| {
        let mut stack = state.stack.clone();

        let invocation = stack.pop_string("execute ");
        let invocation = invocation.trim();
        let (exe, args) = invocation.split_once(" ").unwrap_or((invocation, ""));
        let args = args.split_ascii_whitespace().collect::<Vec<_>>();

        let res = std::process::Command::new(exe).args(args).output().unwrap();

        let mut quot = Stack::new();
        quot.push_i64(res.status.code().unwrap_or(-1).into());
        quot.push_str(String::from_utf8(res.stdout).unwrap().trim_end());
        quot.push_str(String::from_utf8(res.stderr).unwrap().trim_end());

        stack.push_quotation(quot);

        state.update_stack(stack)
    })]
}
