use crate::corelib::run_quot;
use crate::{Context, RailOp, RailState};

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

            // A mini-world for the changes
            let mini_world = RailState {
                stack: working_stack,
                dictionary: state.dictionary.clone(),
                context: Context::None,
            };

            let mini_world = run_quot(&quot, mini_world);

            stack.push_quotation(mini_world.stack);

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
    ]
}
