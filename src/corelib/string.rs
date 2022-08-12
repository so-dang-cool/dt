use crate::rail_machine::{RailDef, RailType, Stack};

use RailType::*;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_state("upcase", &[String], &[String], |quote| {
            let (s, quote) = quote.pop_string("upcase");
            quote.push_string(s.to_uppercase())
        }),
        RailDef::on_state("downcase", &[String], &[String], |quote| {
            let (s, quote) = quote.pop_string("downcase");
            quote.push_string(s.to_lowercase())
        }),
        RailDef::on_state("trim", &[String], &[String], |quote| {
            let (s, quote) = quote.pop_string("trim");
            quote.push_string(s.trim().to_string())
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_state("split", &[String, String], &[Quote], |state| {
            let (delimiter, state) = state.pop_string("split");
            let (s, state) = state.pop_string("split");

            let words = s
                .split(&delimiter)
                .fold(state.child(), |words, word| words.push_str(word));

            state.push_quote(words)
        }),
        // TODO: Should this also work on Quotes?
        RailDef::on_state("join", &[Quote, String], &[String], |quote| {
            let (delimiter, quote) = quote.pop_string("join");
            let (strings, quote) = quote.pop_quote("join");
            quote.push_string(join("join", strings.stack, &delimiter))
        }),
        RailDef::on_state("contains?", &[String, String], &[Boolean], |quote| {
            let (substring, quote) = quote.pop_string("contains?");
            let (string, quote) = quote.pop_string("contains?");
            let is_contained = string.contains(&substring);
            quote.push_bool(is_contained)
        }),
        RailDef::on_state("starts-with?", &[String, String], &[Boolean], |quote| {
            let (prefix, quote) = quote.pop_string("starts-with?");
            let (string, quote) = quote.pop_string("starts-with?");
            let is_prefix = string.starts_with(&prefix);
            quote.push_bool(is_prefix)
        }),
        RailDef::on_state("ends-with?", &[String, String], &[Boolean], |quote| {
            let (suffix, quote) = quote.pop_string("ends-with?");
            let (string, quote) = quote.pop_string("ends-with?");
            let is_suffix = string.ends_with(&suffix);
            quote.push_bool(is_suffix)
        }),
    ]
}

fn join(context: &str, words: Stack, delimiter: &str) -> std::string::String {
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
