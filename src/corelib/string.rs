use crate::rail_machine::{RailOp, Stack};

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
        RailOp::new("split", &["string", "string"], &["quot"], |state| {
            let mut stack = state.stack.clone();
            let delimiter = stack.pop_string("split");
            let s = stack.pop_string("split");
            stack.push_quotation(split(s, &delimiter));
            state.update_stack(stack)
        }),
        RailOp::new("join", &["quot", "string"], &["string"], |state| {
            let mut stack = state.stack.clone();
            let delimiter = stack.pop_string("split");
            let quot = stack.pop_quotation("join");
            stack.push_string(join("join", quot, &delimiter));
            state.update_stack(stack)
        }),
    ]
}

fn string_splitter<'a>(name: &'a str, delimiter: &'a str) -> RailOp<'a> {
    RailOp::new(name, &["string"], &["quot"], move |state| {
        let mut stack = state.stack.clone();
        let s = stack.pop_string(name);
        stack.push_quotation(split(s, delimiter));
        state.update_stack(stack)
    })
}

fn split(s: String, delimiter: &str) -> Stack {
    let mut words = Stack::new();
    s.split(delimiter)
        .map(|s| s.to_string())
        .for_each(|s| words.push_string(s));
    words
}

fn string_joiner<'a>(name: &'a str, delimiter: &'a str) -> RailOp<'a> {
    RailOp::new(name, &["string"], &["quot"], move |state| {
        let mut stack = state.stack.clone();
        let quot = stack.pop_quotation(name);
        stack.push_string(join(name, quot, delimiter));
        state.update_stack(stack)
    })
}

fn join(context: &str, words: Stack, delimiter: &str) -> String {
    let mut s = vec![];
    let mut words = words;
    while !words.is_empty() {
        s.push(words.pop_string(context));
    }
    s.reverse();
    s.join(delimiter)
}
