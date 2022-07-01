use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack("type", &["a"], &["string"], |stack| {
            let (thing, stack) = stack.pop();
            stack.push_string(thing.type_name())
        }),
        RailDef::on_state("dict", &[], &["quote"], |state| {
            state.update_stack_and_dict(|stack, dictionary| {
                let mut defs = dictionary.keys().collect::<Vec<_>>();
                defs.sort();
                let quote = defs
                    .iter()
                    .fold(Quote::default(), |stack, def| stack.push_str(def));
                let stack = stack.push_quote(quote);
                (stack, dictionary)
            })
        }),
        RailDef::on_stack("quote", &["a"], &["quote"], |quote| {
            let (a, quote) = quote.pop();
            let wrapping_quote = Quote::default();
            let wrapping_quote = wrapping_quote.push(a);
            quote.push_quote(wrapping_quote)
        }),
        // TODO: In typing, consumes of 'quote-all' should be something that means 0-to-many
        RailDef::on_stack("quote-all", &[], &["quote"], |prev_quote| {
            let quote = Quote::default();
            quote.push_quote(prev_quote)
        }),
        RailDef::on_stack("unquote", &["quote"], &["..."], |quote| {
            let (wrapping_quote, mut quote) = quote.pop_quote("unquote");
            for value in wrapping_quote.values {
                quote = quote.push(value);
            }
            quote
        }),
    ]
}
