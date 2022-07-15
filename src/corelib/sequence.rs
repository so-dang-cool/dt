use crate::rail_machine::{self, run_quote, Quote, RailDef, RailVal};

// TODO: These should all work for both String and Quote? Should String also be a Quote? Typeclasses?
pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("len", &["quote|string"], &["i64"], |quote| {
            let (a, quote) = quote.pop();
            let len: i64 = match a {
                RailVal::Quote(quote) => quote.len(),
                RailVal::String(s) => s.len(),
                _ => {
                    rail_machine::log_warn(format!(
                        "Can only perform len on quote or string but got {}",
                        a
                    ));
                    return quote.push(a);
                }
            }
            .try_into()
            .unwrap();
            quote.push_i64(len)
        }),
        RailDef::on_quote("quote", &["a"], &["quote"], |quote| {
            let (a, quote) = quote.pop();
            let wrapper = Quote::default();
            let wrapper = wrapper.push(a);
            quote.push_quote(wrapper)
        }),
        RailDef::on_quote("unquote", &["quote"], &["..."], |quote| {
            let (wrapper, mut quote) = quote.pop_quote("unquote");
            for value in wrapper.values {
                quote = quote.push(value);
            }
            quote
        }),
        RailDef::on_quote("push", &["quote", "a"], &["quote"], |quote| {
            let (a, quote) = quote.pop();
            let (sequence, quote) = quote.pop_quote("push");
            let sequence = sequence.push(a);
            quote.push_quote(sequence)
        }),
        RailDef::on_quote("pop", &["quote"], &["quote", "a"], |quote| {
            let (sequence, quote) = quote.pop_quote("pop");
            let (a, sequence) = sequence.pop();
            quote.push_quote(sequence).push(a)
        }),
        RailDef::on_quote("enq", &["a", "quote"], &["quote"], |quote| {
            let (sequence, quote) = quote.pop_quote("push");
            let (a, quote) = quote.pop();
            let sequence = sequence.enqueue(a);
            quote.push_quote(sequence)
        }),
        RailDef::on_quote("deq", &["quote"], &["a", "quote"], |quote| {
            let (sequence, quote) = quote.pop_quote("pop");
            let (a, sequence) = sequence.dequeue();
            quote.push(a).push_quote(sequence)
        }),
        RailDef::on_quote("rev", &["quote"], &["quote"], |quote| {
            let (mut sequence, quote) = quote.pop_quote("rev");
            sequence.values.reverse();
            quote.push_quote(sequence)
        }),
        RailDef::on_quote("concat", &["quote", "quote"], &["quote"], |quote| {
            let (suffix, quote) = quote.pop_quote("concat");
            let (prefix, quote) = quote.pop_quote("concat");
            let mut results = Quote::default();
            for term in prefix.values.into_iter().chain(suffix.values) {
                results = results.push(term);
            }
            quote.push_quote(results)
        }),
        RailDef::on_state("filter", &["quote", "quote"], &["quote"], |state| {
            let (predicate, quote) = state.quote.clone().pop_quote("filter");
            let (sequence, quote) = quote.pop_quote("filter");
            let mut results = Quote::default();

            for term in sequence.values {
                let substate = state.jail_state(Quote::default().push(term.clone()));
                let substate = run_quote(&predicate, substate);
                let (keep, _) = substate.quote.pop_bool("filter");
                if keep {
                    results = results.push(term);
                }
            }

            let quote = quote.push_quote(results);

            state.replace_quote(quote)
        }),
        RailDef::on_state("map", &["quote", "quote"], &["quote"], |state| {
            state.clone().update_quote(move |quote| {
                let (transform, quote) = quote.pop_quote("map");
                let (sequence, quote) = quote.pop_quote("map");
                let mut results = Quote::default();

                for term in sequence.values {
                    results = results.push(term.clone());
                    let substate = state.jail_state(results);
                    let substate = run_quote(&transform, substate);
                    results = substate.quote;
                }

                quote.push_quote(results)
            })
        }),
        RailDef::on_state("each!", &["quote", "quote"], &[], |state| {
            let (command, quote) = state.quote.clone().pop_quote("each");
            let (sequence, quote) = quote.pop_quote("each");

            let state = state.replace_quote(quote);

            sequence.values.into_iter().fold(state, |state, value| {
                let state = state.update_quote(|quote| quote.push(value.clone()));
                run_quote(&command, state)
            })
        }),
        RailDef::on_jailed_state("each", &["quote", "quote"], &[], |state| {
            let (command, quote) = state.quote.clone().pop_quote("each");
            let (sequence, quote) = quote.pop_quote("each");

            let state = state.replace_quote(quote);

            let dictionary = state.dictionary.clone();

            sequence.values.into_iter().fold(state, |state, value| {
                let state = state
                    .update_quote(|quote| quote.push(value.clone()))
                    .replace_dictionary(dictionary.clone());
                run_quote(&command, state)
            })
        }),
    ]
}
