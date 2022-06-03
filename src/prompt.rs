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
                eprintln!("Derailed: {:?}", e);
                // TODO: Stack dump?
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
