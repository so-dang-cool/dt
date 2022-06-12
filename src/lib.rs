pub mod corelib;
pub mod prompt;

use crate::corelib::RailOp;
pub use corelib::operate;
use corelib::{new_dictionary, Dictionary};

#[derive(Clone, Debug)]
pub struct RailState {
    stack: Stack,
    dictionary: Dictionary,
    context: Context,
}

impl RailState {
    pub fn new(context: Context) -> RailState {
        let stack = Stack::new();
        let dictionary = new_dictionary();
        RailState {
            stack,
            dictionary,
            context,
        }
    }

    pub fn update_stack(self, stack: Stack) -> RailState {
        RailState {
            stack,
            dictionary: self.dictionary,
            context: self.context,
        }
    }

    pub fn deeper(self) -> RailState {
        let context = Context::Quotation {
            context: Box::new(self.context),
            parent: Box::new(self.stack),
        };
        RailState {
            stack: Stack::new(),
            dictionary: self.dictionary,
            context,
        }
    }

    pub fn higher(self) -> RailState {
        let (context, mut stack) = match self.context {
            Context::Quotation { context, parent } => (*context, *parent),
            Context::Main => panic!("Can't escape main"),
        };

        stack.push(RailVal::Quotation(self.stack));

        RailState {
            stack,
            dictionary: self.dictionary,
            context,
        }
    }
}

impl Default for RailState {
    fn default() -> Self {
        Self::new(Context::Main)
    }
}

#[derive(Clone, Debug)]
pub enum Context {
    Main,
    Quotation {
        context: Box<Context>,
        parent: Box<Stack>,
    },
}

#[derive(Clone, Debug)]
enum RailVal {
    I64(i64),
    String(String),
    Operator(RailOp<'static>),
    Quotation(Stack),
}

impl std::fmt::Display for RailVal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use RailVal::*;
        match self {
            I64(n) => write!(fmt, "{:?}", n),
            String(s) => write!(fmt, "\"{}\"", s),
            Operator(o) => write!(fmt, "{:?}", o),
            Quotation(q) => write!(fmt, "{:?}", q),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    terms: Vec<RailVal>,
}

impl Stack {
    fn new() -> Self {
        Stack { terms: vec![] }
    }

    fn push(&mut self, term: RailVal) {
        self.terms.push(term)
    }

    fn len(&self) -> usize {
        self.terms.len()
    }

    fn pop(&mut self) -> Option<RailVal> {
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
