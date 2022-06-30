use crate::rail_machine::{run_quot, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state("times", &["quot", "i64"], &[], |state| {
        let (n, stack) = state.stack.clone().pop_i64("times");
        let (quot, stack) = stack.pop_quote("times");
        let state = state.replace_stack(stack);
        (0..n).fold(state, |state, _n| run_quot(&quot, state))
    })]
}
