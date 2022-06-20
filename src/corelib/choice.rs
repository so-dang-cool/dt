use crate::corelib::run_quot;
use crate::{RailOp, Stack};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![RailOp::new("opt", &["seq"], &[], |state| {
        let mut stack = state.stack.clone();

        // TODO: All conditions and all actions must have the same stack effect.
        let mut options = stack.pop_quotation("opt");

        let mut state = state.update_stack(stack);

        while !options.is_empty() {
            let action: Stack = options.pop_quotation("opt");
            let condition: Stack = options.pop_quotation("opt");
            state = run_quot(&condition, state);
            let mut stack = state.stack.clone();
            let success = stack.pop_bool("opt");
            state = state.update_stack(stack);
            if success {
                state = run_quot(&action, state);
                break;
            }
        }

        state
    })]
}
