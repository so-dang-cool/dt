use crate::dt_machine::{self, Definition, DtType};

use DtType::*;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        Definition::on_state("stab", &[], &[Stab], |quote| {
            quote.push_stab(dt_machine::new_stab())
        }),
        Definition::on_state("insert", &[Stab, Quote], &[Stab], |quote| {
            let (k, v, quote) = quote.pop_stab_entry("insert");
            let (mut st, quote) = quote.pop_stab("insert");
            st.insert(k, v);
            quote.push_stab(st)
        }),
        Definition::on_state("extract", &[Stab, String], &[A], |quote| {
            let (k, quote) = quote.pop_string("insert");
            let (st, quote) = quote.pop_stab("insert");
            let result = st.get(&k).unwrap().to_owned();
            quote.push(result)
        }),
    ]
}
