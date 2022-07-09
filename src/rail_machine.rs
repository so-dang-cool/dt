use colored::Colorize;

use crate::corelib::corelib_dictionary;
use crate::prompt::operate_term;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RailState {
    // TODO: Provide update functions and make these private
    pub quote: Quote,
    pub dictionary: Dictionary,
    pub context: Context,
}

impl RailState {
    pub fn new(context: Context) -> RailState {
        let quote = Quote::default();
        let dictionary = corelib_dictionary();
        RailState {
            quote,
            dictionary,
            context,
        }
    }

    pub fn run_tokens(self, tokens: Vec<String>) -> RailState {
        tokens.iter().fold(self, operate_term)
    }

    pub fn update_quote(self, update: impl Fn(Quote) -> Quote) -> RailState {
        RailState {
            quote: update(self.quote),
            dictionary: self.dictionary,
            context: self.context,
        }
    }

    pub fn update_quote_and_dict(
        self,
        update: impl Fn(Quote, Dictionary) -> (Quote, Dictionary),
    ) -> RailState {
        let (quote, dictionary) = update(self.quote, self.dictionary);
        RailState {
            quote,
            dictionary,
            context: self.context,
        }
    }

    pub fn replace_quote(self, quote: Quote) -> RailState {
        RailState {
            quote,
            dictionary: self.dictionary,
            context: self.context,
        }
    }

    /// A substate that will take a parent dictionary, but never leak its own
    /// dictionary to parent contexts.
    pub fn jail_state(&self, quote: Quote) -> RailState {
        RailState {
            quote,
            dictionary: self.dictionary.clone(),
            context: Context::None,
        }
    }

    pub fn deeper(self) -> RailState {
        RailState {
            quote: Quote::default(),
            dictionary: empty_dictionary(),
            context: Context::Quotation {
                parent_state: Box::new(self),
            },
        }
    }

    pub fn in_main(&self) -> bool {
        matches!(self.context, Context::Main)
    }

    pub fn higher(self) -> RailState {
        let state = match self.context {
            Context::Quotation { parent_state } => *parent_state,
            Context::Main => panic!("Can't escape main"),
            Context::None => panic!("Can't escape"),
        };

        let quote = state.quote.clone().push_quote(self.quote);
        state.replace_quote(quote)
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
    Quotation { parent_state: Box<RailState> },
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RailVal {
    Boolean(bool),
    // TODO: Make a "Numeric" typeclass. (And floating-point/rational numbers)
    I64(i64),
    F64(f64),
    Command(String),
    Quote(Quote),
    String(String),
}

impl RailVal {
    pub fn type_name(&self) -> String {
        use RailVal::*;
        match self {
            Boolean(_) => "bool",
            I64(_) => "i64",
            F64(_) => "f64",
            Command(_) => "command",
            Quote(_) => "quote",
            String(_) => "string",
        }
        .into()
    }
}

impl std::fmt::Display for RailVal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use RailVal::*;
        match self {
            Boolean(b) => write!(fmt, "{}", if *b { "true" } else { "false" }),
            I64(n) => write!(fmt, "{}", n),
            F64(n) => write!(fmt, "{}", n),
            Command(o) => write!(fmt, "{}", o),
            Quote(q) => write!(fmt, "{}", q),
            String(s) => write!(fmt, "\"{}\"", s.replace('\n', "\\n")),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quote {
    pub values: Vec<RailVal>,
}

impl Quote {
    pub fn new(values: Vec<RailVal>) -> Self {
        Quote { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(mut self, term: RailVal) -> Quote {
        self.values.push(term);
        self
    }

    pub fn push_bool(mut self, b: bool) -> Quote {
        self.values.push(RailVal::Boolean(b));
        self
    }

    pub fn push_i64(mut self, i: i64) -> Quote {
        self.values.push(RailVal::I64(i));
        self
    }

    pub fn push_f64(mut self, n: f64) -> Quote {
        self.values.push(RailVal::F64(n));
        self
    }

    pub fn push_command(mut self, op_name: &str) -> Quote {
        self.values.push(RailVal::Command(op_name.to_owned()));
        self
    }

    pub fn push_quote(mut self, quote: Quote) -> Quote {
        self.values.push(RailVal::Quote(quote));
        self
    }

    pub fn push_string(mut self, s: String) -> Quote {
        self.values.push(RailVal::String(s));
        self
    }

    pub fn push_str(mut self, s: &str) -> Quote {
        self.values.push(RailVal::String(s.to_owned()));
        self
    }

    pub fn pop(mut self) -> (RailVal, Quote) {
        let term = self.values.pop().unwrap();
        (term, self)
    }

    pub fn pop_bool(mut self, context: &str) -> (bool, Quote) {
        match self.values.pop().unwrap() {
            RailVal::Boolean(b) => (b, self),
            rail_val => panic!("{}", type_panic_msg(context, "bool", rail_val)),
        }
    }

    pub fn pop_i64(mut self, context: &str) -> (i64, Quote) {
        match self.values.pop().unwrap() {
            RailVal::I64(n) => (n, self),
            rail_val => panic!("{}", type_panic_msg(context, "i64", rail_val)),
        }
    }

    pub fn pop_f64(mut self, context: &str) -> (f64, Quote) {
        match self.values.pop().unwrap() {
            RailVal::F64(n) => (n, self),
            rail_val => panic!("{}", type_panic_msg(context, "f64", rail_val)),
        }
    }

    fn _pop_command(mut self, context: &str) -> (String, Quote) {
        match self.values.pop().unwrap() {
            RailVal::Command(op) => (op, self),
            rail_val => panic!("{}", type_panic_msg(context, "command", rail_val)),
        }
    }

    pub fn pop_quote(mut self, context: &str) -> (Quote, Quote) {
        match self.values.pop().unwrap() {
            RailVal::Quote(quote) => (quote, self),
            rail_val => panic!("{}", type_panic_msg(context, "quote", rail_val)),
        }
    }

    pub fn pop_string(mut self, context: &str) -> (String, Quote) {
        match self.values.pop().unwrap() {
            RailVal::String(s) => (s, self),
            rail_val => panic!("{}", type_panic_msg(context, "string", rail_val)),
        }
    }

    pub fn enqueue(mut self, value: RailVal) -> Quote {
        self.values.insert(0, value);
        self
    }

    pub fn dequeue(mut self) -> (RailVal, Quote) {
        let value = self.values.remove(0);
        (value, self)
    }
}

impl Default for Quote {
    fn default() -> Self {
        let values = vec![];
        Self::new(values)
    }
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[ ").unwrap();

        for term in &self.values {
            write!(f, "{} ", term).unwrap();
        }

        write!(f, "]").unwrap();

        Ok(())
    }
}

pub type Dictionary = HashMap<String, RailDef<'static>>;

#[derive(Clone)]
pub struct RailDef<'a> {
    pub name: String,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    action: RailAction<'a>,
}

#[derive(Clone)]
pub enum RailAction<'a> {
    Builtin(Arc<dyn Fn(RailState) -> RailState + 'a>),
    Quotation(Quote),
}

impl RailDef<'_> {
    pub fn on_state<'a, F>(
        name: &str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        state_action: F,
    ) -> RailDef<'a>
    where
        F: Fn(RailState) -> RailState + 'a,
    {
        RailDef {
            name: name.to_string(),
            consumes,
            produces,
            action: RailAction::Builtin(Arc::new(state_action)),
        }
    }

    pub fn on_quote<'a, F>(
        name: &str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        quote_action: F,
    ) -> RailDef<'a>
    where
        F: Fn(Quote) -> Quote + 'a,
    {
        RailDef::on_state(name, consumes, produces, move |state| {
            state.update_quote(&quote_action)
        })
    }

    pub fn contextless<'a, F>(
        name: &str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        contextless_action: F,
    ) -> RailDef<'a>
    where
        F: Fn() + 'a,
    {
        RailDef::on_state(name, consumes, produces, move |state| {
            contextless_action();
            state
        })
    }

    pub fn from_quote<'a>(name: &str, quote: Quote) -> RailDef<'a> {
        // TODO: Infer quote effects
        RailDef {
            name: name.to_string(),
            consumes: &[],
            produces: &[],
            action: RailAction::Quotation(quote),
        }
    }

    pub fn act(&mut self, state: RailState) -> RailState {
        if state.quote.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            log_derail(format!(
                "Derailed: quote underflow for \"{}\" ({} -> {}): quote only had {}",
                self.name,
                self.consumes.join(" "),
                self.produces.join(" "),
                state.quote.len()
            ));
            std::process::exit(1);
        }

        // TODO: Type checks

        match &self.action {
            RailAction::Builtin(action) => action(state),
            RailAction::Quotation(quote) => run_quote(quote, state),
        }
    }
}

impl Debug for RailDef<'_> {
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

pub fn run_quote(quote: &Quote, state: RailState) -> RailState {
    quote
        .values
        .iter()
        .fold(state, |state, rail_val| match rail_val {
            RailVal::Command(op_name) => {
                if let Some(op) = state.dictionary.get(&op_name.clone()) {
                    op.clone().act(state)
                } else {
                    log_warn(format!("Skipping undefined term: {}", op_name));
                    state
                }
            }
            _ => state.update_quote(|quote| quote.push(rail_val.clone())),
        })
}

pub fn empty_dictionary() -> Dictionary {
    HashMap::new()
}

// The following are all formatting things for errors/warnings/panics.
// TODO: Update places these are referenced to return Result.

pub fn type_panic_msg(context: &str, expected: &str, actual: RailVal) -> String {
    format!(
        "[Context: {}] Wanted {}, but got {}",
        context, expected, actual
    )
}

pub fn log_warn(thing: impl Display) {
    let msg = format!("WARN: {}", thing).dimmed().red();
    eprintln!("{}", msg);
}

pub fn log_derail(thing: impl Display) {
    let msg = format!("Derailed: {}", thing).dimmed().red();
    eprintln!("{}", msg);
}
