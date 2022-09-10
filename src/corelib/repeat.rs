use crate::dt_machine::{Definition, DtType};

use DtType::*;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![Definition::on_state("times", &[Quote, I64], &[], |state| {
        let (n, stack) = state.stack.clone().pop_i64("times");
        let (commands, stack) = stack.pop_quote("times");
        let state = state.replace_stack(stack);
        (0..n).fold(state, |state, _n| {
            commands.clone().jailed_run_in_state(state)
        })
    })]
}
