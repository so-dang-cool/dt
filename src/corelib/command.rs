use crate::rail_machine::{self, RailDef, RailState, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do!", &["quote|command"], &["..."], do_it()),
        RailDef::on_jailed_state("do", &["quote|command"], &["..."], do_it()),
        RailDef::on_state("doin!", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_jailed_state("doin", &["quote", "quote|command"], &["..."], doin()),
        RailDef::on_state("def!", &["quote", "string|command"], &[], |state| {
            state.update_stack_and_defs(|quote, definitions| {
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
            let (name, state) = state.pop();
            let name = if let Some(name) = get_command_name(&name) {
                name
            } else {
                rail_machine::log_warn(format!("{} is not a string or command", name));
                return state;
            };
            let is_def = state.definitions.contains_key(&name);
            state.push_bool(is_def)
        }),
    ]
}

fn do_it() -> impl Fn(RailState) -> RailState {
    |state| {
        let (commands, quote) = state.stack.clone().pop();
        let state = state.replace_stack(quote);

        match commands {
            RailVal::Quote(quote) => quote.run_in_state(state),
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
        let (commands, state) = state.pop_quote("doin");
        let (targets, state) = state.pop_quote("doin");

        let substate = state.child().replace_stack(targets.stack);
        let substate = commands.run_in_state(substate);

        state.push_quote(substate)
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
