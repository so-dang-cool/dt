use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("type", &["a"], &["string"], |quote| {
            let (thing, quote) = quote.pop();
            quote.push_string(thing.type_name())
        }),
        RailDef::on_state("defs", &[], &["quote"], |state| {
            state.update_quote_and_dict(|quote, dictionary| {
                let mut defs = dictionary.keys().collect::<Vec<_>>();
                defs.sort();
                let defs = defs
                    .iter()
                    .fold(Quote::default(), |quote, def| quote.push_str(def));
                let quote = quote.push_quote(defs);
                (quote, dictionary)
            })
        }),
        RailDef::on_quote("quote", &["a"], &["quote"], |quote| {
            let (a, quote) = quote.pop();
            let wrapper = Quote::default();
            let wrapper = wrapper.push(a);
            quote.push_quote(wrapper)
        }),
        // TODO: In typing, consumes of 'quote-all' should be something that means 0-to-many
        RailDef::on_quote("quote-all", &[], &["quote"], |quote| {
            let wrapper = Quote::default();
            wrapper.push_quote(quote)
        }),
        RailDef::on_quote("unquote", &["quote"], &["..."], |quote| {
            let (wrapper, mut quote) = quote.pop_quote("unquote");
            for value in wrapper.values {
                quote = quote.push(value);
            }
            quote
        }),
    ]
}
