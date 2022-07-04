use colored::Colorize;

use crate::rail_machine::{run_quote, RailDef, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do", &["quote|command"], &["..."], |state| {
            let (a, quote) = state.quote.clone().pop();
            let state = state.replace_quote(quote);

            match a {
                RailVal::Quote(quote) => run_quote(&quote, state),
                RailVal::Command(command) => {
                    let action = state.dictionary.get(&command).unwrap();
                    action.clone().act(state)
                }
                _ => panic!("oops"),
            }
        }),
        RailDef::on_state("doin", &["quote", "quote"], &["quote"], |state| {
            state.clone().update_quote(|quote| {
                let (commands, quote) = quote.pop_quote("doin");
                let (targets, quote) = quote.pop_quote("doin");

                let substate = state.contextless_child(targets); // TODO: Really just need dictionaries.
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
                    let msg = format!("ERROR: {} was already defined.", name).dimmed().red();
                    eprintln!("{}", msg);
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
    ]
}
