use crate::rail_machine::{run_quot, type_panic_msg, Quote, RailDef, RailVal};

// TODO: Should all these work for a String too? Should String also be a stack?
pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("len", &["quot|string"], &["i64"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let len: i64 = match a {
                    RailVal::Quote(quot) => quot.len(),
                    RailVal::String(s) => s.len(),
                    _ => panic!("{}", type_panic_msg("len", "quot|string", a)),
                }
                .try_into()
                .unwrap();
                stack.push_i64(len)
            })
        }),
        RailDef::on_state("push", &["quot", "a"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let (quot, stack) = stack.pop_quote("push");
                let quot = quot.push(a);
                stack.push_quote(quot)
            })
        }),
        RailDef::on_state("pop", &["quot"], &["quot", "a"], |state| {
            state.update_stack(|stack| {
                let (quot, stack) = stack.pop_quote("pop");
                let (a, quot) = quot.pop();
                stack.push_quote(quot).push(a)
            })
        }),
        RailDef::on_state("rev", &["quot"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (mut quot, stack) = stack.pop_quote("rev");
                quot.values.reverse();
                stack.push_quote(quot)
            })
        }),
        RailDef::on_state("concat", &["quot", "quot"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (quot_b, stack) = stack.pop_quote("concat");
                let (quot_a, stack) = stack.pop_quote("concat");
                let mut quot = Quote::new();
                for term in quot_a.values.into_iter().chain(quot_b.values) {
                    quot = quot.push(term);
                }
                stack.push_quote(quot)
            })
        }),
        RailDef::on_state("filter", &["quot", "quot"], &["quot"], |state| {
            let (predicate, stack) = state.stack.clone().pop_quote("filter");
            let (supply_stack, stack) = stack.pop_quote("filter");
            let mut result_stack = Quote::new();

            for term in supply_stack.values {
                let substate = state.contextless_child(Quote::new().push(term.clone()));
                let substate = run_quot(&predicate, substate);
                let (keep, _) = substate.stack.pop_bool("filter");
                if keep {
                    result_stack = result_stack.push(term);
                }
            }

            let stack = stack.push_quote(result_stack);

            state.replace_stack(stack)
        }),
        RailDef::on_state("map", &["quot", "quot"], &["quot"], |state| {
            state.clone().update_stack(move |stack| {
                let (transform, stack) = stack.pop_quote("map");
                let (supply_stack, stack) = stack.pop_quote("map");
                let mut result_stack = Quote::new();

                for term in supply_stack.values {
                    result_stack = result_stack.push(term.clone());
                    let substate = state.contextless_child(result_stack);
                    let substate = run_quot(&transform, substate);
                    result_stack = substate.stack;
                }

                stack.push_quote(result_stack)
            })
        }),
        RailDef::on_state("each", &["quot", "quot"], &[], |state| {
            let (action, stack) = state.stack.clone().pop_quote("each");
            let (supply_stack, stack) = stack.pop_quote("each");
            let state = state.replace_stack(stack);

            supply_stack.values.into_iter().fold(state, |state, value| {
                let state = state.update_stack(|stack| stack.push(value.clone()));
                run_quot(&action, state)
            })
        }),
    ]
}
