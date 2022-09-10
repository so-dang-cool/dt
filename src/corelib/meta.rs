use crate::dt_machine::{Context, Definition, DtType};

use DtType::*;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        Definition::on_state("type", &[A], &[String], |quote| {
            let (thing, quote) = quote.pop();
            quote.push_string(thing.type_name())
        }),
        Definition::on_state("defs", &[], &[Quote], |state| {
            let mut defs = state.definitions.keys().collect::<Vec<_>>();
            defs.sort();

            let defs = defs
                .iter()
                .fold(state.child(), |quote, def| quote.push_str(def));

            state.push_quote(defs)
        }),
        // TODO: In typing, consumes of 'quote-all' should be something that means 0-to-many
        Definition::on_state("quote-all", &[], &[Quote], |quote| {
            let wrapper = quote.child().replace_context(Context::Main);
            let quote = quote.replace_context(Context::Quotation {
                parent_state: Box::new(wrapper.clone()),
            });
            wrapper.push_quote(quote)
        }),
    ]
}
