use std::fmt::Display;

use crate::rail_machine::RailState;
use crate::{tokens, RAIL_VERSION};
use colored::{ColoredString, Colorize};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn operate_term<S>(state: RailState, term: S) -> RailState
where
    S: Into<String>,
{
    let term: String = term.into();
    let mut quote = state.quote.clone();
    let dictionary = state.dictionary.clone();

    // Quotations
    if term == "[" {
        return state.deeper();
    } else if term == "]" {
        return state.higher();
    }
    // Defined operations
    else if let Some(op) = dictionary.get(&term) {
        if state.in_main() {
            return op.clone().act(state.clone());
        } else {
            quote = quote.push_command(&op.name);
        }
    }
    // Strings
    else if term.starts_with('"') && term.ends_with('"') {
        let term = term.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        quote = quote.push_string(term.to_string());
    }
    // Integers
    else if let Ok(i) = term.parse::<i64>() {
        quote = quote.push_i64(i);
    }
    // Floating point numbers
    else if let Ok(n) = term.parse::<f64>() {
        quote = quote.push_f64(n);
    }
    // Unknown
    else if !state.in_main() {
        quote = quote.push_command(&term)
    } else {
        // TODO: Use a logging library? Log levels? Exit in a strict mode?
        // TODO: Have/get details on filename/source, line number, character number
        let message = format!(
            "WARN: Skipping unknown term: \"{}\"",
            term.replace('\n', "\\n")
        )
        .dimmed()
        .red();
        eprintln!("{}", message);
    }

    RailState {
        quote,
        dictionary,
        context: state.context,
    }
}

pub struct RailPrompt {
    editor: Editor<()>,
    terms: Vec<String>,
}

impl RailPrompt {
    pub fn new() -> RailPrompt {
        let editor = Editor::<()>::new();
        let terms = vec![];
        RailPrompt { editor, terms }
    }

    pub fn run(self, state: RailState) {
        let name_and_version = format!("rail {}", RAIL_VERSION);
        eprintln!("{}", name_and_version.dimmed().red());

        let end_state = self.fold(state, operate_term);

        let end_state_msg = format!("State dump: {}", end_state.quote);
        eprintln!("{}", end_state_msg.dimmed().red());
    }
}

impl Default for RailPrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for RailPrompt {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        while self.terms.is_empty() {
            let input = self.editor.readline("> ");

            if let Err(e) = input {
                // ^D and ^C are not error cases.
                if let ReadlineError::Eof = e {
                    eprintln!("{}", derail_msg("End of input"));
                    return None;
                } else if let ReadlineError::Interrupted = e {
                    eprintln!("{}", derail_msg("Process interrupt"));
                    return None;
                }

                eprintln!("{}", derail_msg(e));
                std::process::exit(1);
            }

            let input = input.unwrap();

            self.editor.add_history_entry(&input);

            self.terms = tokens::from_rail_source(input);
            self.terms.reverse();
        }

        self.terms.pop()
    }
}

fn derail_msg(msg: impl Display) -> ColoredString {
    format!("Derailed: {}", msg).dimmed().red()
}
