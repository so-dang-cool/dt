use crate::rail_machine::{RailDef, RailType};

use RailType::*;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state("times", &[Quote, I64], &[], |state| {
        let (n, stack) = state.stack.clone().pop_i64("times");
        let (commands, stack) = stack.pop_quote("times");
        let state = state.replace_stack(stack);
        (0..n).fold(state, |state, _n| {
            commands.clone().jailed_run_in_state(state)
        })
    })]
}
