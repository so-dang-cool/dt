use crate::rail_machine::{self, run_quote, RailDef, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do", &["quote|command"], &["..."], |state| {
            let (a, quote) = state.quote.clone().pop();
            let state = state.replace_quote(quote);

            match a {
                RailVal::Quote(quote) => run_quote(&quote, state),
                RailVal::Command(command) => {
                    let state = state.clone().get_def(&command).unwrap().clone().act(state);
                    state
                }
                _ => {
                    rail_machine::log_warn(format!("{} is not a quote or command", a));
                    state
                }
            }
        }),
        RailDef::on_state("doin", &["quote", "quote"], &["quote"], |state| {
            state.clone().update_quote(|quote| {
                let (commands, quote) = quote.pop_quote("doin");
                let (targets, quote) = quote.pop_quote("doin");

                let substate = state.jail_state(targets); // TODO: Really just need dictionaries.
                let substate = run_quote(&commands, substate);

                quote.push_quote(substate.quote)
            })
        }),
        RailDef::on_state("def", &["quote", "string"], &[], |state| {
            state.update_quote_and_dict(|quote, dictionary| {
                let mut dictionary = dictionary;
                let (name, quote) = quote.pop_string("def");
                let (commands, quote) = quote.pop_quote("def");
                if dictionary.contains_key(&name) {
                    rail_machine::log_warn(format!("{} was already defined.", name));
                    return (quote, dictionary);
                }
                dictionary.insert(name.clone(), RailDef::from_quote(&name, commands));
                (quote, dictionary)
            })
        }),
        RailDef::on_state("def?", &["string"], &["bool"], |state| {
            state.clone().update_quote(|quote| {
                let (name, quote) = quote.pop_string("def?");
                quote.push_bool(state.dictionary.contains_key(&name))
            })
        }),
        RailDef::on_state("tmp-def", &["quote", "string"], &[], |state| {
            state.update_quote_and_temp(|quote, temp_dictionary| {
                let mut temp_dictionary = temp_dictionary;
                let (name, quote) = quote.pop_string("tmp-def");
                let (commands, quote) = quote.pop_quote("tmp-def");
                temp_dictionary.insert(name.clone(), RailDef::from_quote(&name, commands));
                (quote, temp_dictionary)
            })
        }),
        RailDef::on_state("tmp-undef", &["string"], &[], |state| {
            state.update_quote_and_temp(|quote, temp_dictionary| {
                let mut temp_dictionary = temp_dictionary;
                let (name, quote) = quote.pop_string("tmp-def");
                temp_dictionary.remove(&name);
                (quote, temp_dictionary)
            })
        }),
    ]
}
