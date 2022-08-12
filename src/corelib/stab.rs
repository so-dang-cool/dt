use crate::rail_machine::{self, RailDef, RailType};

use RailType::*;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("stab", &[], &[Stab], |quote| {
            quote.push_stab(rail_machine::new_stab())
        }),
        RailDef::on_state("insert", &[Stab, Quote], &[Stab], |quote| {
            let (k, v, quote) = quote.pop_stab_entry("insert");
            let (mut st, quote) = quote.pop_stab("insert");
            st.insert(k, v);
            quote.push_stab(st)
        }),
        RailDef::on_state("extract", &[Stab, String], &[A], |quote| {
            let (k, quote) = quote.pop_string("insert");
            let (st, quote) = quote.pop_stab("insert");
            let result = st.get(&k).unwrap().to_owned();
            quote.push(result)
        }),
    ]
}
