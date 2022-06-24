use crate::corelib::new_dictionary;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RailState {
    // TODO: Provide update functions and make these private
    pub stack: Stack,
    pub dictionary: Dictionary,
    pub context: Context,
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

    pub fn contextless_child(&self, stack: Stack) -> RailState {
        RailState {
            stack,
            dictionary: self.dictionary.clone(),
            context: Context::None,
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

    pub fn in_main(&self) -> bool {
        matches!(self.context, Context::Main)
    }

    pub fn higher(self) -> RailState {
        let (context, mut stack) = match self.context {
            Context::Quotation { context, parent } => (*context, *parent),
            Context::Main => panic!("Can't escape main"),
            Context::None => panic!("Can't escape"),
        };

        stack.push_quotation(self.stack);

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
    None,
}

#[derive(Clone, Debug)]
pub enum RailVal {
    Boolean(bool),
    // TODO: Make a "Numeric" typeclass. (And floating-point/rational numbers)
    I64(i64),
    Operator(RailOp<'static>),
    Quotation(Stack),
    String(String),
}

impl std::fmt::Display for RailVal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use RailVal::*;
        match self {
            Boolean(b) => write!(fmt, "{}", if *b { "true" } else { "false" }),
            I64(n) => write!(fmt, "{}", n),
            Operator(o) => write!(fmt, "{}", o.name),
            Quotation(q) => write!(fmt, "{}", q),
            String(s) => write!(fmt, "\"{}\"", s.replace('\n', "\\n")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub terms: Vec<RailVal>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { terms: vec![] }
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn push(&mut self, term: RailVal) {
        self.terms.push(term)
    }

    pub fn push_bool(&mut self, b: bool) {
        self.terms.push(RailVal::Boolean(b))
    }

    pub fn push_i64(&mut self, i: i64) {
        self.terms.push(RailVal::I64(i))
    }

    pub fn push_operator(&mut self, op: RailOp<'static>) {
        self.terms.push(RailVal::Operator(op))
    }

    pub fn push_quotation(&mut self, quot: Stack) {
        self.terms.push(RailVal::Quotation(quot))
    }

    pub fn push_string(&mut self, s: String) {
        self.terms.push(RailVal::String(s))
    }

    pub fn push_str(&mut self, s: &str) {
        self.terms.push(RailVal::String(s.to_owned()))
    }

    pub fn pop(&mut self) -> Option<RailVal> {
        self.terms.pop()
    }

    pub fn pop_bool(&mut self, context: &str) -> bool {
        match self.terms.pop().unwrap() {
            RailVal::Boolean(b) => b,
            rail_val => panic!("{}", type_panic_msg(context, "boolean", rail_val)),
        }
    }

    pub fn pop_i64(&mut self, context: &str) -> i64 {
        match self.terms.pop().unwrap() {
            RailVal::I64(n) => n,
            rail_val => panic!("{}", type_panic_msg(context, "i64", rail_val)),
        }
    }

    fn _pop_operator(&mut self, context: &str) -> RailOp<'static> {
        match self.terms.pop().unwrap() {
            RailVal::Operator(op) => op,
            rail_val => panic!("{}", type_panic_msg(context, "operator", rail_val)),
        }
    }

    pub fn pop_quotation(&mut self, context: &str) -> Stack {
        match self.terms.pop().unwrap() {
            RailVal::Quotation(quot) => quot,
            rail_val => panic!("{}", type_panic_msg(context, "quotation", rail_val)),
        }
    }

    pub fn pop_string(&mut self, context: &str) -> String {
        match self.terms.pop().unwrap() {
            RailVal::String(s) => s,
            rail_val => panic!("{}", type_panic_msg(context, "string", rail_val)),
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[ ").unwrap();

        for term in &self.terms {
            write!(f, "{} ", term).unwrap();
        }

        write!(f, "]").unwrap();

        Ok(())
    }
}

fn type_panic_msg(context: &str, expected: &str, actual: RailVal) -> String {
    format!(
        "[Context: {}] Wanted {}, but got {}",
        context, expected, actual
    )
}

pub type Dictionary = HashMap<String, RailOp<'static>>;

#[derive(Clone)]
pub struct RailOp<'a> {
    pub name: String,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    action: RailAction<'a>,
}

#[derive(Clone)]
pub enum RailAction<'a> {
    Builtin(Arc<dyn Fn(RailState) -> RailState + 'a>),
    Quotation(Stack),
}

impl RailOp<'_> {
    pub fn new<'a, F>(
        name: &str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: F,
    ) -> RailOp<'a>
    where
        F: Fn(RailState) -> RailState + 'a,
    {
        RailOp {
            name: name.to_string(),
            consumes,
            produces,
            action: RailAction::Builtin(Arc::new(op)),
        }
    }

    pub fn from_quot<'a>(name: &str, quot: Stack) -> RailOp<'a> {
        // TODO: Infer stack effects
        RailOp {
            name: name.to_string(),
            consumes: &[],
            produces: &[],
            action: RailAction::Quotation(quot),
        }
    }

    pub fn go(&mut self, state: RailState) -> RailState {
        if state.stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            eprintln!(
                "Derailed: stack underflow for \"{}\" ({} -> {}): stack only had {}",
                self.name,
                self.consumes.join(" "),
                self.produces.join(" "),
                state.stack.len()
            );
            std::process::exit(1);
        }

        // TODO: Type checks

        match &self.action {
            RailAction::Builtin(op) => op(state),
            RailAction::Quotation(quot) => run_quot(quot, state),
        }
    }
}

impl Debug for RailOp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} ({} -> {})",
            self.name,
            self.consumes.join(" "),
            self.produces.join(" ")
        )
    }
}

pub fn run_quot(quot: &Stack, state: RailState) -> RailState {
    quot.terms.iter().fold(state, |state, rail_val| {
        let mut stack = state.stack.clone();
        match rail_val {
            RailVal::Operator(op) => {
                let mut op = op.clone();
                return op.go(state);
            }
            _ => stack.push(rail_val.clone()),
        }
        state.update_stack(stack)
    })
}
