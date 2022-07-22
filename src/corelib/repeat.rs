use crate::rail_machine::{run_quote, RailDef, RailState};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("times!", &["quote", "i64"], &[], times()),
        RailDef::on_jailed_state("times", &["quote", "i64"], &[], times()),
    ]
}

fn times() -> impl Fn(RailState) -> RailState {
    |state| {
        let (n, quote) = state.values.clone().pop_i64("times");
        let (commands, quote) = quote.pop_quote("times");
        let state = state.replace_values(quote);
        (0..n).fold(state, |state, _n| run_quote(&commands, state))
    }
}
