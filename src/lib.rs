pub mod corelib;
pub mod prompt;

pub use corelib::operate;
use corelib::{new_dictionary, Dictionary};

pub struct RailState {
    stack: Stack,
    dictionary: Dictionary,
}

impl RailState {
    pub fn new() -> RailState {
        let stack = Stack::new();
        let dictionary = new_dictionary();
        RailState { stack, dictionary }
    }
}

impl Default for RailState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
enum RailTerm {
    I64(i64),
    String(String),
}

impl std::fmt::Display for RailTerm {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use RailTerm::*;
        match self {
            I64(n) => write!(fmt, "{:?}", n),
            String(s) => write!(fmt, "\"{}\"", s),
        }
    }
}

struct Stack {
    terms: Vec<RailTerm>,
}

impl Stack {
    fn new() -> Self {
        Stack { terms: vec![] }
    }

    fn push(&mut self, term: RailTerm) {
        self.terms.push(term)
    }

    fn len(&self) -> usize {
        self.terms.len()
    }

    fn pop(&mut self) -> Option<RailTerm> {
        self.terms.pop()
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for term in &self.terms {
            term.fmt(f).unwrap();
            write!(f, " ").unwrap();
        }
        Ok(())
    }
}
