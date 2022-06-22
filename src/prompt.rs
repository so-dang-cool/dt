use crate::RailState;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn operate(state: RailState, term: String) -> RailState {
    let mut stack = state.stack.clone();
    let dictionary = state.dictionary.clone();
    let context = state.context.clone();

    // Comments
    if state.in_comment() {
        if term == "*/" {
            return state.exit_comment();
        }
        return state;
    } else if term == "/*" {
        return state.enter_comment();
    }

    // Strings
    // TODO: Need a better way to soak up exact whitespace characters.
    if state.in_string() {
        if term == "\"" {
            return state.append_string("").exit_string();
        } else if term.ends_with('"') {
            let term = term.strip_suffix('"').unwrap();
            return state.append_string(term).exit_string();
        }
        return state.append_string(&term);
    } else if term.starts_with('"') {
        if term == "\"" {
            return state.enter_string();
        } else if term.ends_with('"') {
            let term = term.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
            return state.enter_string().append_string_exact(term).exit_string();
        }
        let term = term.strip_prefix('"').unwrap();
        return state.enter_string().append_string_exact(term);
    }

    // Quotations
    if term == "[" {
        return state.deeper();
    } else if term == "]" {
        return state.higher();
    }
    // Defined operations
    else if let Some(op) = dictionary.get(&term) {
        if state.in_main() {
            return op.clone().go(state.clone());
        } else {
            stack.push_operator(op.clone());
        }
    }
    // Integers
    else if let Ok(i) = term.parse::<i64>() {
        stack.push_i64(i);
    }
    // Unknown
    else {
        eprintln!("Derailed: unknown term {:?}", term);
        std::process::exit(1);
    }

    RailState {
        stack,
        dictionary,
        context,
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

    pub fn run(self) {
        let end_state = self.fold(RailState::default(), operate);

        println!("{}", end_state.stack);
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

                eprintln!("Derailed: {:?}", e);
                std::process::exit(1);
            }

            let input = input.unwrap();

            self.editor.add_history_entry(&input);

            self.terms = input
                .split_whitespace()
                .map(|s| s.to_owned())
                .rev()
                .collect::<Vec<_>>();
        }

        self.terms.pop()
    }
}
