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
        string_splitter("words", " "),
        string_joiner("unwords", " "),
        string_splitter("chars", ""),
        string_splitter("lines", "\n"),
        string_joiner("unlines", "\n"),
    ]
}

pub fn string_splitter<'a>(name: &'a str, delimiter: &'a str) -> RailOp<'a> {
    RailOp::new(name, &["string"], &["quot"], move |state| {
        let mut stack = state.stack.clone();
        let s = stack.pop_string(name);
        let mut words = Stack::new();
        s.split(delimiter)
            .for_each(|word| words.push_string(word.to_string()));
        stack.push_quotation(words);
        state.update_stack(stack)
    })
}

pub fn string_joiner<'a>(name: &'a str, delimiter: &'a str) -> RailOp<'a> {
    RailOp::new(name, &["string"], &["quot"], move |state| {
        let mut stack = state.stack.clone();
        let mut quot = stack.pop_quotation(name);
        let mut s = vec![];
        while !quot.is_empty() {
            s.push(quot.pop_string(name));
        }
        s.reverse();
        stack.push_string(s.join(delimiter));
        state.update_stack(stack)
    })
}
