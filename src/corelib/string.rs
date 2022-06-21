use crate::RailOp;
use crate::Stack;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("upcase", &["string"], &["string"], |state| {
            let mut stack = state.stack.clone();
            let s = stack.pop_string("upcase");
            stack.push_string(s.to_uppercase());
            state.update_stack(stack)
        }),
        RailOp::new("downcase", &["string"], &["string"], |state| {
            let mut stack = state.stack.clone();
            let s = stack.pop_string("downcase");
            stack.push_string(s.to_lowercase());
            state.update_stack(stack)
        }),
        RailOp::new("trim", &["string"], &["string"], |state| {
            let mut stack = state.stack.clone();
            let s = stack.pop_string("trim");
            stack.push_string(s.trim().to_string());
            state.update_stack(stack)
        }),
        RailOp::new("words", &["string"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let s = stack.pop_string("words");
            let mut words = Stack::new();
            s.split_ascii_whitespace()
                .for_each(|word| words.push_string(word.to_string()));
            stack.push_quotation(words);
            state.update_stack(stack)
        }),
        RailOp::new("unwords", &["quot"], &["string"], |state| {
            let mut stack = state.stack.clone();
            let mut quot = stack.pop_quotation("unwords");
            let mut s = vec![];
            while !quot.is_empty() {
                s.push(quot.pop_string("unwords"));
            }
            s.reverse();
            stack.push_string(s.join(" "));
            state.update_stack(stack)
        }),
    ]
}
