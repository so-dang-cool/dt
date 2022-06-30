mod rustyline_helper;

use crate::rail_machine::RailState;
use crate::tokens;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use rustyline_helper::RailTracks;

pub fn operate_term(state: RailState, term: String) -> RailState {
    let mut stack = state.stack.clone();
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
            stack = stack.push_operator(&op.name);
        }
    }
    // Strings
    else if term.starts_with('"') && term.ends_with('"') {
        let term = term.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        stack = stack.push_string(term.to_string());
    }
    // Integers
    else if let Ok(i) = term.parse::<i64>() {
        stack = stack.push_i64(i);
    }
    // Floating point numbers
    else if let Ok(n) = term.parse::<f64>() {
        stack = stack.push_f64(n);
    }
    // Unknown
    else if !state.in_main() {
        stack = stack.push_operator(&term)
    } else {
        eprintln!("Derailed: unknown term \"{}\"", term);
        std::process::exit(1);
    }

    RailState {
        stack,
        dictionary,
        context: state.context,
    }
}

pub struct RailPrompt {
    editor: Editor<RailTracks>,
    terms: Vec<String>,
}

impl RailPrompt {
    pub fn new() -> RailPrompt {
        let editor = Editor::<RailTracks>::new();
        let terms = vec![];
        RailPrompt { editor, terms }
    }

    pub fn run(&mut self) {
        let mut state = RailState::default();

        self.editor.set_helper(Some(RailTracks::for_state(&state)));

        let mut maybe_term = self.next();

        while let Some(term) = maybe_term {
            state = operate_term(state, term);
            self.editor.set_helper(Some(RailTracks::for_state(&state)));
            maybe_term = self.next();
        }

        println!("{}", state.stack);
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
