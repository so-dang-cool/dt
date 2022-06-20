use crate::corelib::run_quot;
use crate::{RailOp, RailState};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("call", &["quot"], &["..."], |state| {
            let mut stack = state.stack.clone();
            let quot = stack.pop_quotation("call");
            let state = state.update_stack(stack);
            run_quot(&quot, state)
        }),
        RailOp::new("def", &["quot", "s"], &[], |state| {
            let mut stack = state.stack;
            let name = stack.pop_string("def");
            let quot = stack.pop_quotation("def");
            let mut dictionary = state.dictionary;
            dictionary.insert(name.clone(), RailOp::from_quot(&name, quot));
            RailState {
                stack,
                dictionary,
                context: state.context,
            }
        }),
    ]
}
