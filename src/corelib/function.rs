use crate::corelib::run_quot;
use crate::{RailOp, RailState, Stack};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("call", &["quot"], &["..."], |state| {
            let mut stack = state.stack.clone();
            let quot = stack.pop_quotation("call");
            let state = state.update_stack(stack);
            run_quot(&quot, state)
        }),
        RailOp::new("call-in", &["quot", "quot"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let quot = stack.pop_quotation("call-in");
            let working_stack = stack.pop_quotation("call-in");

            let substate = state.contextless_child(working_stack);
            let substate = run_quot(&quot, substate);

            stack.push_quotation(substate.stack);

            state.update_stack(stack)
        }),
        RailOp::new("def", &["quot", "s"], &[], |state| {
            let mut stack = state.stack;
            let name = stack.pop_string("def");
            let quot = stack.pop_quotation("def");
            let mut dictionary = state.dictionary;
            dictionary.insert(name.clone(), RailOp::from_quot(&name, quot));
            RailState {
                stack,
                dictionary,
                context: state.context,
            }
        }),
        RailOp::new("def?", &["s"], &["bool"], |state| {
            let mut stack = state.stack.clone();
            let name = stack.pop_string("def?");
            stack.push_bool(state.dictionary.contains_key(&name));
            state.update_stack(stack)
        }),
        RailOp::new("undef", &["s"], &[], |state| {
            let mut stack = state.stack;
            let name = stack.pop_string("undef");
            let mut dictionary = state.dictionary;
            dictionary
                .remove(&name)
                .unwrap_or_else(|| panic!("Cannot undef \"{}\", it was already undefined", name));
            RailState {
                stack,
                dictionary,
                context: state.context,
            }
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
    ]
}
