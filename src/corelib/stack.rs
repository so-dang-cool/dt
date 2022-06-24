use crate::rail_machine::{run_quot, RailDef, Stack};

// TODO: Should all these work for a String too? Should String also be a stack?
pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("len", &["quot"], &["i64"], |state| {
            state.update_stack(|stack| {
                let (quot, stack) = stack.pop_quotation("len");
                let len: i64 = quot.len().try_into().unwrap();
                stack.push_i64(len)
            })
        }),
        RailDef::on_state("push", &["quot", "a"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let (quot, stack) = stack.pop_quotation("push");
                let quot = quot.push(a);
                stack.push_quotation(quot)
            })
        }),
        RailDef::on_state("pop", &["quot"], &["quot", "a"], |state| {
            state.update_stack(|stack| {
                let (quot, stack) = stack.pop_quotation("pop");
                let (a, quot) = quot.pop();
                stack.push_quotation(quot).push(a)
            })
        }),
        RailDef::on_state("rev", &["quot"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (mut quot, stack) = stack.pop_quotation("rev");
                quot.values.reverse();
                stack.push_quotation(quot)
            })
        }),
        RailDef::on_state("concat", &["quot", "quot"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (quot_b, stack) = stack.pop_quotation("concat");
                let (quot_a, stack) = stack.pop_quotation("concat");
                let mut quot = Stack::new();
                for term in quot_a.values.into_iter().chain(quot_b.values) {
                    quot = quot.push(term);
                }
                stack.push_quotation(quot)
            })
        }),
        RailDef::on_state("filter", &["quot", "quot"], &["quot"], |state| {
            let (predicate, stack) = state.stack.clone().pop_quotation("filter");
            let (supply_stack, stack) = stack.pop_quotation("filter");
            let mut result_stack = Stack::new();

            for term in supply_stack.values {
                let substate = state.contextless_child(Stack::new().push(term.clone()));
                let substate = run_quot(&predicate, substate);
                let (keep, _) = substate.stack.pop_bool("filter");
                if keep {
                    result_stack = result_stack.push(term);
                }
            }

            let stack = stack.push_quotation(result_stack);

            state.replace_stack(stack)
        }),
        RailDef::on_state("map", &["quot", "quot"], &["quot"], |state| {
            state.clone().update_stack(move |stack| {
                let (transform, stack) = stack.pop_quotation("map");
                let (supply_stack, stack) = stack.pop_quotation("map");
                let mut result_stack = Stack::new();

                for term in supply_stack.values {
                    result_stack = result_stack.push(term.clone());
                    let substate = state.contextless_child(result_stack);
                    let substate = run_quot(&transform, substate);
                    result_stack = substate.stack;
                }

                stack.push_quotation(result_stack)
            })
        }),
        RailDef::on_state("each", &["quot", "quot"], &[], |state| {
            let (action, stack) = state.stack.clone().pop_quotation("each");
            let (supply_stack, stack) = stack.pop_quotation("each");
            let state = state.replace_stack(stack);

            supply_stack.values.into_iter().fold(state, |state, value| {
                let state = state.update_stack(|stack| stack.push(value.clone()));
                run_quot(&action, state)
            })
        }),
    ]
}
