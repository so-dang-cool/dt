use crate::rail_machine::{RailDef, RailType};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state("opt", &[RailType::Quote], &[], |state| {
        // TODO: All conditions and all actions must have the same quote effect.
        let (options, quote) = state.stack.clone().pop_quote("opt");
        let mut state = state.replace_stack(quote);

        let mut options = options.reverse();

        while !options.is_empty() {
            let (condition, opts) = options.pop_quote("opt");
            let (action, opts) = opts.pop_quote("opt");
            options = opts;

            state = condition.jailed_run_in_state(state);
            let (success, quote) = state.stack.clone().pop_bool("opt");
            state = state.replace_stack(quote);

            if success {
                return action.run_in_state(state);
            }
        }

        state
    })]
}
