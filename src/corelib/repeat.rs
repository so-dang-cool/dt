use crate::rail_machine::{run_quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_state(
        "times",
        &["quote", "i64"],
        &[],
        |state| {
            let (n, quote) = state.quote.clone().pop_i64("times");
            let (commands, quote) = quote.pop_quote("times");
            let state = state.replace_quote(quote);
            (0..n).fold(state, |state, _n| run_quote(&commands, state))
        },
    )]
}
