use crate::RailState;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
        let end_state = self.fold(RailState::default(), super::operate);

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
