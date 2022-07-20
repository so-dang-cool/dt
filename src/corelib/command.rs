use crate::rail_machine::{self, run_quote, RailDef, RailState, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do!", &["quote|command"], &["..."], do_it()),
        RailDef::on_jailed_state("do", &["quote|command"], &["..."], do_it()),
        RailDef::on_state("doin!", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_jailed_state("doin", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_state("def!", &["quote", "string|command"], &[], |state| {
            state.update_quote_and_dict(|quote, dictionary| {
                let mut dictionary = dictionary;
                let (name, quote) = quote.pop();
                let name = if let Some(name) = get_command_name(&name) {
                    name
                } else {
                    rail_machine::log_warn(format!("{} is not a string or command", name));
                    return (quote, dictionary);
                };
                let (commands, quote) = quote.pop_quote("def");
                if dictionary.contains_key(&name) {
                    rail_machine::log_warn(format!("{} was already defined.", name));
                    return (quote, dictionary);
                }
                dictionary.insert(name.clone(), RailDef::from_quote(&name, commands));
                (quote, dictionary)
            })
        }),
        RailDef::on_state("def?", &["string|command"], &["bool"], |state| {
            state.clone().update_quote(|quote| {
                let (name, quote) = quote.pop();
                let name = if let Some(name) = get_command_name(&name) {
                    name
                } else {
                    rail_machine::log_warn(format!("{} is not a string or command", name));
                    return quote;
                };
                quote.push_bool(state.dictionary.contains_key(&name))
            })
        }),
    ]
}

fn do_it() -> impl Fn(RailState) -> RailState {
    |state| {
        let (commands, quote) = state.quote.clone().pop();
        let state = state.replace_quote(quote);

        match commands {
            RailVal::Quote(quote) => run_quote(&quote, state),
            RailVal::Command(command) => {
                let action = state.dictionary.get(&command).unwrap();
                action.clone().act(state)
            }
            _ => {
                rail_machine::log_warn(format!("{} is not a quote or command", commands));
                state
            }
        }
    }
}

fn doin() -> impl Fn(RailState) -> RailState {
    |state| {
        state.clone().update_quote(|quote| {
            let (commands, quote) = quote.pop_quote("doin");
            let (targets, quote) = quote.pop_quote("doin");

            let substate = state.jail_state(targets); // TODO: Really just need dictionaries.
            let substate = run_quote(&commands, substate);

            quote.push_quote(substate.quote)
        })
    }
}

fn get_command_name(name: &RailVal) -> Option<String> {
    match name.clone() {
        RailVal::String(s) => Some(s),
        RailVal::Command(c) => Some(c),
        RailVal::Quote(q) => {
            let (v, q) = q.pop();
            match (v, q.len()) {
                (RailVal::String(s), 0) => Some(s),
                (RailVal::Command(c), 0) => Some(c),
                _ => None,
            }
        }
        _ => None,
    }
}
