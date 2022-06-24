use crate::rail_machine::{run_quot, RailOp, Stack};

// TODO: Should all these work for a String too? Should String also be a stack?
pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("len", &["quot"], &["i64"], |state| {
            let mut stack = state.stack.clone();
            let quot = stack.pop_quotation("len");
            let len: i64 = quot.len().try_into().unwrap();
            stack.push_i64(len);
            state.update_stack(stack)
        }),
        RailOp::new("push", &["quot", "a"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            let mut quot = stack.pop_quotation("push");
            quot.push(a);
            stack.push_quotation(quot);
            state.update_stack(stack)
        }),
        RailOp::new("pop", &["quot"], &["quot", "a"], |state| {
            let mut stack = state.stack.clone();
            let mut quot = stack.pop_quotation("pop");
            let a = quot.pop().unwrap();
            stack.push_quotation(quot);
            stack.push(a);
            state.update_stack(stack)
        }),
        RailOp::new("rev", &["quot"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let mut quot = stack.pop_quotation("rev");
            quot.terms.reverse();
            stack.push_quotation(quot);
            state.update_stack(stack)
        }),
        RailOp::new("concat", &["quot", "quot"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let quot_b = stack.pop_quotation("concat");
            let quot_a = stack.pop_quotation("concat");
            let mut quot = Stack::new();
            quot_a.terms.into_iter().for_each(|term| quot.push(term));
            quot_b.terms.into_iter().for_each(|term| quot.push(term));
            stack.push_quotation(quot);
            state.update_stack(stack)
        }),
        RailOp::new("filter", &["quot", "quot"], &["quot"], |state| {
            let mut stack = state.stack.clone();

            let predicate = stack.pop_quotation("filter");
            let supply_stack = stack.pop_quotation("filter");
            let mut result_stack = Stack::new();

            for term in supply_stack.terms {
                let mut working_stack = Stack::new();
                working_stack.push(term.clone());
                let substate = state.contextless_child(working_stack);
                let substate = run_quot(&predicate, substate);
                let mut working_stack = substate.stack;
                let keep = working_stack.pop_bool("filter");
                if keep {
                    result_stack.push(term);
                }
            }

            stack.push_quotation(result_stack);

            state.update_stack(stack)
        }),
        RailOp::new("map", &["quot", "quot"], &["quot"], |state| {
            let mut stack = state.stack.clone();

            let transform = stack.pop_quotation("map");
            let supply_stack = stack.pop_quotation("map");
            let mut result_stack = Stack::new();

            for term in supply_stack.terms {
                result_stack.push(term.clone());
                let substate = state.contextless_child(result_stack);
                let substate = run_quot(&transform, substate);
                result_stack = substate.stack;
            }

            stack.push_quotation(result_stack);

            state.update_stack(stack)
        }),
    ]
}
