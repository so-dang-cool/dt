use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("upcase", &["string"], &["string"], |state| {
            state.update_stack(|stack| {
                let (s, stack) = stack.pop_string("upcase");
                stack.push_string(s.to_uppercase())
            })
        }),
        RailDef::on_state("downcase", &["string"], &["string"], |state| {
            state.update_stack(|stack| {
                let (s, stack) = stack.pop_string("downcase");
                stack.push_string(s.to_lowercase())
            })
        }),
        RailDef::on_state("trim", &["string"], &["string"], |state| {
            state.update_stack(|stack| {
                let (s, stack) = stack.pop_string("trim");
                stack.push_string(s.trim().to_string())
            })
        }),
        RailDef::on_state("split", &["string", "string"], &["quot"], |state| {
            state.update_stack(|stack| {
                let (delimiter, stack) = stack.pop_string("split");
                let (s, stack) = stack.pop_string("split");
                stack.push_quote(split(s, &delimiter))
            })
        }),
        RailDef::on_state("join", &["quot", "string"], &["string"], |state| {
            state.update_stack(|stack| {
                let (delimiter, stack) = stack.pop_string("join");
                let (quot, stack) = stack.pop_quote("join");
                stack.push_string(join("join", quot, &delimiter))
            })
        }),
    ]
}

fn split(s: String, delimiter: &str) -> Quote {
    let mut words = Quote::new();
    for s in s.split(delimiter) {
        words = words.push_str(s);
    }
    words
}

fn join(context: &str, words: Quote, delimiter: &str) -> String {
    let mut s = vec![];
    let mut words = words;
    while !words.is_empty() {
        let (part, new_words) = words.pop_string(context);
        s.push(part);
        words = new_words
    }
    s.reverse();
    s.join(delimiter)
}
