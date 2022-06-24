use crate::rail_machine::{run_quot, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state("opt", &["seq"], &[], |state| {
        // TODO: All conditions and all actions must have the same stack effect.
        let (mut options, stack) = state.stack.clone().pop_quotation("opt");
        let mut state = state.replace_stack(stack);

        while !options.is_empty() {
            let (action, opts) = options.pop_quotation("opt");
            let (condition, opts) = opts.pop_quotation("opt");
            options = opts;

            state = run_quot(&condition, state);
            let (success, stack) = state.stack.clone().pop_bool("opt");
            state = state.replace_stack(stack);

            if success {
                return run_quot(&action, state);
            }
        }

        state
    })]
}
