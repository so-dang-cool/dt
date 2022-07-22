use crate::rail_machine::{RailDef, Stack};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("type", &["a"], &["string"], |quote| {
            let (thing, quote) = quote.pop();
            quote.push_string(thing.type_name())
        }),
        RailDef::on_state("defs", &[], &["quote"], |state| {
            state.update_values_and_defs(|quote, definitions| {
                let mut defs = definitions.keys().collect::<Vec<_>>();
                defs.sort();
                let defs = defs
                    .iter()
                    .fold(Stack::default(), |quote, def| quote.push_str(def));
                let quote = quote.push_quote(defs);
                (quote, definitions)
            })
        }),
        // TODO: In typing, consumes of 'quote-all' should be something that means 0-to-many
        RailDef::on_quote("quote-all", &[], &["quote"], |quote| {
            let wrapper = Stack::default();
            wrapper.push_quote(quote)
        }),
    ]
}
