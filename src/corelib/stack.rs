use crate::rail_machine::{run_quote, type_panic_msg, Quote, RailDef, RailVal};

// TODO: Should all these work for a String too? Should String also be a stack?
pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("len", &["quote|string"], &["i64"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let len: i64 = match a {
                    RailVal::Quote(quote) => quote.len(),
                    RailVal::String(s) => s.len(),
                    _ => panic!("{}", type_panic_msg("len", "quote|string", a)),
                }
                .try_into()
                .unwrap();
                stack.push_i64(len)
            })
        }),
        RailDef::on_state("push", &["quote", "a"], &["quote"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let (quote, stack) = stack.pop_quote("push");
                let quote = quote.push(a);
                stack.push_quote(quote)
            })
        }),
        RailDef::on_state("pop", &["quote"], &["quote", "a"], |state| {
            state.update_stack(|stack| {
                let (quote, stack) = stack.pop_quote("pop");
                let (a, quote) = quote.pop();
                stack.push_quote(quote).push(a)
            })
        }),
        RailDef::on_state("rev", &["quote"], &["quote"], |state| {
            state.update_stack(|stack| {
                let (mut quote, stack) = stack.pop_quote("rev");
                quote.values.reverse();
                stack.push_quote(quote)
            })
        }),
        RailDef::on_state("concat", &["quote", "quote"], &["quote"], |state| {
            state.update_stack(|stack| {
                let (quote_b, stack) = stack.pop_quote("concat");
                let (quote_a, stack) = stack.pop_quote("concat");
                let mut quote = Quote::new();
                for term in quote_a.values.into_iter().chain(quote_b.values) {
                    quote = quote.push(term);
                }
                stack.push_quote(quote)
            })
        }),
        RailDef::on_state("filter", &["quote", "quote"], &["quote"], |state| {
            let (predicate, stack) = state.stack.clone().pop_quote("filter");
            let (supply_stack, stack) = stack.pop_quote("filter");
            let mut result_stack = Quote::new();

            for term in supply_stack.values {
                let substate = state.contextless_child(Quote::new().push(term.clone()));
                let substate = run_quote(&predicate, substate);
                let (keep, _) = substate.stack.pop_bool("filter");
                if keep {
                    result_stack = result_stack.push(term);
                }
            }

            let stack = stack.push_quote(result_stack);

            state.replace_stack(stack)
        }),
        RailDef::on_state("map", &["quote", "quote"], &["quote"], |state| {
            state.clone().update_stack(move |stack| {
                let (transform, stack) = stack.pop_quote("map");
                let (supply_stack, stack) = stack.pop_quote("map");
                let mut result_stack = Quote::new();

                for term in supply_stack.values {
                    result_stack = result_stack.push(term.clone());
                    let substate = state.contextless_child(result_stack);
                    let substate = run_quote(&transform, substate);
                    result_stack = substate.stack;
                }

                stack.push_quote(result_stack)
            })
        }),
        RailDef::on_state("each", &["quote", "quote"], &[], |state| {
            let (action, stack) = state.stack.clone().pop_quote("each");
            let (supply_stack, stack) = stack.pop_quote("each");
            let state = state.replace_stack(stack);

            supply_stack.values.into_iter().fold(state, |state, value| {
                let state = state.update_stack(|stack| stack.push(value.clone()));
                run_quote(&action, state)
            })
        }),
    ]
}
