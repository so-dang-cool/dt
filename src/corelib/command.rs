use crate::rail_machine::{run_quote, RailDef, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("do", &["quote|function"], &["..."], |state| {
            let (a, stack) = state.stack.clone().pop();
            let state = state.replace_stack(stack);

            match a {
                RailVal::Quote(quote) => run_quote(&quote, state),
                RailVal::Command(function) => {
                    let action = state.dictionary.get(&function).unwrap();
                    action.clone().act(state)
                }
                _ => panic!("oops"),
            }
        }),
        RailDef::on_state("doin", &["quote", "quote"], &["quote"], |state| {
            state.clone().update_stack(|stack| {
                let (quote, stack) = stack.pop_quote("doin");
                let (working_stack, stack) = stack.pop_quote("doin");

                let substate = state.contextless_child(working_stack); // TODO: Really just need dictionary.
                let substate = run_quote(&quote, substate);

                stack.push_quote(substate.stack)
            })
        }),
        RailDef::on_state("def", &["quote", "string"], &[], |state| {
            state.update_stack_and_dict(|stack, dictionary| {
                let mut dictionary = dictionary;
                let (name, stack) = stack.pop_string("def");
                let (quote, stack) = stack.pop_quote("def");
                dictionary.insert(name.clone(), RailDef::from_quote(&name, quote));
                (stack, dictionary)
            })
        }),
        RailDef::on_state("def?", &["string"], &["bool"], |state| {
            state.clone().update_stack(|stack| {
                let (name, stack) = stack.pop_string("def?");
                stack.push_bool(state.dictionary.contains_key(&name))
            })
        }),
        RailDef::on_state("undef", &["string"], &[], |state| {
            state.update_stack_and_dict(|stack, dictionary| {
                let mut dictionary = dictionary;
                let (name, stack) = stack.pop_string("undef");
                dictionary.remove(&name).unwrap_or_else(|| {
                    panic!("Cannot undef \"{}\", it was already undefined", name)
                });
                (stack, dictionary)
            })
        }),
    ]
}
