use crate::rail_machine::{run_quot, RailOp, Stack};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![RailOp::new("opt", &["seq"], &[], |state| {
        // TODO: All conditions and all actions must have the same stack effect.
        let (mut options, stack) = state.stack.clone().pop_quotation("opt");
        let state = state.replace_stack(stack);

        while !options.is_empty() {
            let (action, opts) = options.pop_quotation("opt");
            let (condition, opts) = opts.pop_quotation("opt");
            options = opts;

            let substate = run_quot(&condition, state.contextless_child(Stack::default()));
            let (success, _) = substate.stack.pop_bool("opt");

            if success {
                return run_quot(&action, state);
            }
        }

        state
    })]
}
