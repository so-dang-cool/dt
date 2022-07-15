use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("upcase", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("upcase");
            quote.push_string(s.to_uppercase())
        }),
        RailDef::on_quote("downcase", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("downcase");
            quote.push_string(s.to_lowercase())
        }),
        RailDef::on_quote("trim", &["string"], &["string"], |quote| {
            let (s, quote) = quote.pop_string("trim");
            quote.push_string(s.trim().to_string())
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_quote("split", &["string", "string"], &["quote"], |quote| {
            let (delimiter, quote) = quote.pop_string("split");
            let (s, quote) = quote.pop_string("split");
            quote.push_quote(split(s, &delimiter))
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_quote("join", &["quote", "string"], &["string"], |quote| {
            let (delimiter, quote) = quote.pop_string("join");
            let (strings, quote) = quote.pop_quote("join");
            quote.push_string(join("join", strings, &delimiter))
        }),
    ]
}

fn split(s: String, delimiter: &str) -> Quote {
    let mut words = Quote::default();
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
