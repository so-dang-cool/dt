use crate::rail_machine::{RailDef, RailState, Stack};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("upcase", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("upcase");
            quote.push_string(s.to_uppercase())
        }),
        RailDef::on_state("downcase", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("downcase");
            quote.push_string(s.to_lowercase())
        }),
        RailDef::on_state("trim", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("trim");
            quote.push_string(s.trim().to_string())
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_state("split", &["string", "string"], &["quote"], |state| {
            state.clone().update_stack(|quote| {
                let (delimiter, quote) = quote.pop_string("split");
                let (s, quote) = quote.pop_string("split");
                quote.push_quote(split(state.clone(), s, &delimiter))
            })
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_state("join", &["quote", "string"], &["string"], |quote| {
            let (delimiter, quote) = quote.pop_string("join");
            let (strings, quote) = quote.pop_quote("join");
            quote.push_string(join("join", strings.stack, &delimiter))
        }),
    ]
}

fn split(state: RailState, s: String, delimiter: &str) -> RailState {
    let mut words = Stack::default();
    for s in s.split(delimiter) {
        words = words.push_str(s);
    }
    state.replace_stack(words)
}

fn join(context: &str, words: Stack, delimiter: &str) -> String {
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
