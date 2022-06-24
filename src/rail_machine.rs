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

    pub fn update_stack(self, update: impl Fn(Stack) -> Stack) -> RailState {
        RailState {
            stack: update(self.stack),
            dictionary: self.dictionary,
            context: self.context,
        }
    }

    pub fn update_stack_and_dict(
        self,
        update: impl Fn(Stack, Dictionary) -> (Stack, Dictionary),
    ) -> RailState {
        let (stack, dictionary) = update(self.stack, self.dictionary);
        RailState {
            stack,
            dictionary,
            context: self.context,
        }
    }

    pub fn replace_stack(self, stack: Stack) -> RailState {
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
            parent_context: Box::new(self.context),
            parent_stack: Box::new(self.stack),
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
        let (context, stack) = match self.context {
            Context::Quotation {
                parent_context: context,
                parent_stack: parent,
            } => (*context, *parent),
            Context::Main => panic!("Can't escape main"),
            Context::None => panic!("Can't escape"),
        };

        let stack = stack.push_quotation(self.stack);

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
        parent_context: Box<Context>,
        parent_stack: Box<Stack>,
    },
    None,
}

#[derive(Clone, Debug)]
pub enum RailVal {
    Boolean(bool),
    // TODO: Make a "Numeric" typeclass. (And floating-point/rational numbers)
    I64(i64),
    Operator(String),
    Quotation(Stack),
    String(String),
}

impl RailVal {
    pub fn type_name(&self) -> String {
        use RailVal::*;
        match self {
            Boolean(_) => "bool",
            I64(_) => "i64",
            Operator(_) => "operator",
            Quotation(_) => "quotation",
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
            Operator(o) => write!(fmt, "{}", o),
            Quotation(q) => write!(fmt, "{}", q),
            String(s) => write!(fmt, "\"{}\"", s.replace('\n', "\\n")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub values: Vec<RailVal>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: vec![] }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(mut self, term: RailVal) -> Stack {
        self.values.push(term);
        self
    }

    pub fn push_bool(mut self, b: bool) -> Stack {
        self.values.push(RailVal::Boolean(b));
        self
    }

    pub fn push_i64(mut self, i: i64) -> Stack {
        self.values.push(RailVal::I64(i));
        self
    }

    pub fn push_operator(mut self, op_name: &str) -> Stack {
        self.values.push(RailVal::Operator(op_name.to_owned()));
        self
    }

    pub fn push_quotation(mut self, quot: Stack) -> Stack {
        self.values.push(RailVal::Quotation(quot));
        self
    }

    pub fn push_string(mut self, s: String) -> Stack {
        self.values.push(RailVal::String(s));
        self
    }

    pub fn push_str(mut self, s: &str) -> Stack {
        self.values.push(RailVal::String(s.to_owned()));
        self
    }

    pub fn pop(mut self) -> (RailVal, Stack) {
        let term = self.values.pop().unwrap();
        (term, self)
    }

    pub fn pop_bool(mut self, context: &str) -> (bool, Stack) {
        match self.values.pop().unwrap() {
            RailVal::Boolean(b) => (b, self),
            rail_val => panic!("{}", type_panic_msg(context, "bool", rail_val)),
        }
    }

    pub fn pop_i64(mut self, context: &str) -> (i64, Stack) {
        match self.values.pop().unwrap() {
            RailVal::I64(n) => (n, self),
            rail_val => panic!("{}", type_panic_msg(context, "i64", rail_val)),
        }
    }

    fn _pop_operator(mut self, context: &str) -> (String, Stack) {
        match self.values.pop().unwrap() {
            RailVal::Operator(op) => (op, self),
            rail_val => panic!("{}", type_panic_msg(context, "operator", rail_val)),
        }
    }

    pub fn pop_quotation(mut self, context: &str) -> (Stack, Stack) {
        match self.values.pop().unwrap() {
            RailVal::Quotation(quot) => (quot, self),
            rail_val => panic!("{}", type_panic_msg(context, "quotation", rail_val)),
        }
    }

    pub fn pop_string(mut self, context: &str) -> (String, Stack) {
        match self.values.pop().unwrap() {
            RailVal::String(s) => (s, self),
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

        for term in &self.values {
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
    Quotation(Stack),
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

    pub fn on_stack<'a, F>(
        name: &str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        stack_action: F,
    ) -> RailDef<'a>
    where
        F: Fn(Stack) -> Stack + 'a,
    {
        RailDef::on_state(name, consumes, produces, move |state| {
            state.update_stack(&stack_action)
        })
    }

    pub fn from_quot<'a>(name: &str, quot: Stack) -> RailDef<'a> {
        // TODO: Infer stack effects
        RailDef {
            name: name.to_string(),
            consumes: &[],
            produces: &[],
            action: RailAction::Quotation(quot),
        }
    }

    pub fn act(&mut self, state: RailState) -> RailState {
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
            RailAction::Builtin(action) => action(state),
            RailAction::Quotation(quot) => run_quot(quot, state),
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

pub fn run_quot(quot: &Stack, state: RailState) -> RailState {
    quot.values
        .iter()
        .fold(state, |state, rail_val| match rail_val {
            RailVal::Operator(op_name) => {
                let op = state
                    .dictionary
                    .get(&op_name.clone())
                    .unwrap_or_else(|| panic!("Tried to do \"{}\" but it was undefined", op_name));
                op.clone().act(state)
            }
            _ => state.update_stack(|stack| stack.push(rail_val.clone())),
        })
}
