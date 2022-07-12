use crate::rail_machine::{self, Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("stab", &[], &["stab"], |quote| {
            quote.push_stab(rail_machine::new_stab())
        }),
        RailDef::on_quote("insert", &["stab", "quote"], &["stab"], |quote| {
            let (k, v, quote) = quote.pop_stab_entry("insert");
            let (mut st, quote) = quote.pop_stab("insert");
            st.insert(k, v);
            quote.push_stab(st)
        }),
        RailDef::on_quote("extract", &["stab", "string"], &["quote"], |quote| {
            let (k, quote) = quote.pop_string("insert");
            let (st, quote) = quote.pop_stab("insert");
            let result = if let Some(v) = st.get(&k) {
                Quote::of(v.clone())
            } else {
                Quote::default()
            };
            quote.push_quote(result)
        }),
    ]
}
