use crate::rail_machine::{self, run_quote, RailDef, RailState, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do!", &["quote|command"], &["..."], do_it()),
        RailDef::on_jailed_state("do", &["quote|command"], &["..."], do_it()),
        RailDef::on_state("doin!", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_jailed_state("doin", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_state("def!", &["quote", "string|command"], &[], |state| {
            state.update_values_and_defs(|quote, definitions| {
                let mut definitions = definitions;
                let (name, quote) = quote.pop();
                let name = if let Some(name) = get_command_name(&name) {
                    name
                } else {
                    rail_machine::log_warn(format!("{} is not a string or command", name));
                    return (quote, definitions);
                };
                let (commands, quote) = quote.pop_quote("def");
                if definitions.contains_key(&name) {
                    rail_machine::log_warn(format!("{} was already defined.", name));
                    return (quote, definitions);
                }
                definitions.insert(name.clone(), RailDef::from_quote(&name, commands));
                (quote, definitions)
            })
        }),
        RailDef::on_state("def?", &["string|command"], &["bool"], |state| {
            state.clone().update_values(|quote| {
                let (name, quote) = quote.pop();
                let name = if let Some(name) = get_command_name(&name) {
                    name
                } else {
                    rail_machine::log_warn(format!("{} is not a string or command", name));
                    return quote;
                };
                quote.push_bool(state.definitions.contains_key(&name))
            })
        }),
    ]
}

fn do_it() -> impl Fn(RailState) -> RailState {
    |state| {
        let (commands, quote) = state.values.clone().pop();
        let state = state.replace_values(quote);

        match commands {
            RailVal::Quote(quote) => run_quote(&quote, state),
            RailVal::Command(command) => {
                let action = state.get_def(&command).unwrap();
                action.clone().act(state.clone())
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
        state.clone().update_values(|quote| {
            let (commands, quote) = quote.pop_quote("doin");
            let (targets, quote) = quote.pop_quote("doin");

            let substate = state.jail_state(targets); // TODO: Really just need dictionaries.
            let substate = run_quote(&commands, substate);

            quote.push_quote(substate)
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
