use crate::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("times", &["quot", "i64"], &[], |state| {
            let mut stack = state.stack.clone();
            let n = stack.pop_i64("times");
            let quot = stack.pop_quotation("times");
            let state = state.update_stack(stack);
            (0..n).fold(state, |state, _n| run_quot(&quot, state))
        }),
    ]
}