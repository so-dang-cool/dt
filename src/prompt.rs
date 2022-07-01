use crate::rail_machine::RailState;
use crate::{tokens, RAIL_VERSION};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn operate_term(state: RailState, term: String) -> RailState {
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
        eprintln!("Derailed: unknown term \"{}\"", term.replace('\n', "\\n"));
        std::process::exit(1);
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
        println!("rail {}", RAIL_VERSION);

        let end_state = self.fold(state, operate_term);

        println!("{}", end_state.quote);
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
                    eprintln!("Derailed: End of input");
                    return None;
                } else if let ReadlineError::Interrupted = e {
                    eprintln!("Derailed: Process interrupt");
                    return None;
                }

                eprintln!("Derailed: {}", e);
                std::process::exit(1);
            }

            let input = input.unwrap();

            self.editor.add_history_entry(&input);

            self.terms = tokens::tokenize(&input);
            self.terms.reverse();
        }

        self.terms.pop()
    }
}
