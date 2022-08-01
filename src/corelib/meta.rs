use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("type", &["a"], &["string"], |quote| {
            let (thing, quote) = quote.pop();
            quote.push_string(thing.type_name())
        }),
        RailDef::on_state("defs", &[], &["quote"], |state| {
            let mut defs = state.definitions.keys().collect::<Vec<_>>();
            defs.sort();

            let defs = defs
                .iter()
                .fold(state.child(), |quote, def| quote.push_str(def));

            state.push_quote(defs)
        }),
        // TODO: In typing, consumes of 'quote-all' should be something that means 0-to-many
        RailDef::on_state("quote-all", &[], &["quote"], |quote| {
            let wrapper = quote.child();
            wrapper.push_quote(quote)
        }),
    ]
}
