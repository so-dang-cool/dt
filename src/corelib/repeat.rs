use crate::rail_machine::{run_quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state(
        "times",
        &["quote", "i64"],
        &[],
        |state| {
            let (n, stack) = state.stack.clone().pop_i64("times");
            let (quote, stack) = stack.pop_quote("times");
            let state = state.replace_stack(stack);
            (0..n).fold(state, |state, _n| run_quote(&quote, state))
        },
    )]
}
