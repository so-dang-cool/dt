use colored::Colorize;

use crate::corelib::corelib_dictionary;
use crate::loading;
use im::{HashMap, Vector};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub struct DtState {
    // TODO: Provide update functions and make these private
    pub stack: Stack,
    pub definitions: Dictionary,
    // TODO: Save parents at time of definition and at runtime
    pub context: Context,
}

impl DtState {
    pub fn new(context: Context) -> DtState {
        let stack = Stack::default();
        let definitions = corelib_dictionary();
        DtState {
            stack,
            definitions,
            context,
        }
    }

    pub fn new_with_libs(skip_stdlib: bool, lib_list: Option<String>) -> DtState {
        let state = DtState::default();

        let state = if skip_stdlib {
            state
        } else {
            let tokens = loading::from_stdlib();
            state.run_tokens(tokens)
        };

        if let Some(lib_list) = lib_list {
            let tokens = loading::from_lib_list(&lib_list);
            state.run_tokens(tokens)
        } else {
            state
        }
    }

    pub fn in_main(&self) -> bool {
        matches!(self.context, Context::Main)
    }

    pub fn get_def(&self, name: &str) -> Option<Definition> {
        self.definitions.get(name).cloned()
    }

    pub fn child(&self) -> Self {
        DtState {
            stack: Stack::default(),
            definitions: self.definitions.clone(),
            context: Context::None,
        }
    }

    pub fn run_tokens(self, tokens: Vec<String>) -> DtState {
        tokens.iter().fold(self, |state, term| state.run_term(term))
    }

    pub fn run_term<S>(self, term: S) -> DtState
    where
        S: Into<String>,
    {
        let term: String = term.into();

        // Quotations
        if term == "[" {
            self.deeper()
        } else if term == "]" {
            self.higher()
        }
        // Defined operations
        else if let Some(op) = self.clone().get_def(&term) {
            if self.in_main() {
                let mut op = op;
                op.act(self)
            } else {
                self.push_command(&op.name)
            }
        }
        // Strings
        else if term.starts_with('"') && term.ends_with('"') {
            let term = term.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
            self.push_str(term)
        }
        // Integers
        else if let Ok(i) = term.parse::<i64>() {
            self.push_i64(i)
        }
        // Floating point numbers
        else if let Ok(n) = term.parse::<f64>() {
            self.push_f64(n)
        }
        // Unknown
        else if !self.in_main() {
            // We optimistically expect this may be a not-yet-defined term. This
            // gives a way to do recursive definitions.
            self.push_command(&term)
        } else {
            // TODO: Use a logging library? Log levels? Exit in a strict mode?
            // TODO: Have/get details on filename/source, line number, character number
            let term = term.replace('\n', "\\n");
            exit_for_unknown_command(&term);
        }
    }

    pub fn run_val(&self, value: DtValue, local_state: DtState) -> DtState {
        match value {
            DtValue::Command(name) => {
                let mut cmd = self
                    .get_def(&name)
                    .or_else(|| local_state.get_def(&name))
                    .unwrap_or_else(|| exit_for_unknown_command(&name));
                cmd.act(self.clone())
            }
            value => self.clone().push(value),
        }
    }

    pub fn run_in_state(self, other_state: DtState) -> DtState {
        let values = self.stack.clone().values;
        values.into_iter().fold(other_state, |state, value| {
            state.run_val(value, self.child())
        })
    }

    pub fn jailed_run_in_state(self, other_state: DtState) -> DtState {
        let after_run = self.run_in_state(other_state.clone());
        other_state.replace_stack(after_run.stack)
    }

    pub fn update_stack(self, update: impl Fn(Stack) -> Stack) -> DtState {
        DtState {
            stack: update(self.stack),
            definitions: self.definitions,
            context: self.context,
        }
    }

    pub fn update_stack_and_defs(
        self,
        update: impl Fn(Stack, Dictionary) -> (Stack, Dictionary),
    ) -> DtState {
        let (stack, definitions) = update(self.stack, self.definitions);
        DtState {
            stack,
            definitions,
            context: self.context,
        }
    }

    pub fn replace_stack(self, stack: Stack) -> DtState {
        DtState {
            stack,
            definitions: self.definitions,
            context: self.context,
        }
    }

    pub fn replace_definitions(self, definitions: Dictionary) -> DtState {
        DtState {
            stack: self.stack,
            definitions,
            context: self.context,
        }
    }

    pub fn replace_context(self, context: Context) -> DtState {
        DtState {
            stack: self.stack,
            definitions: self.definitions,
            context,
        }
    }

    pub fn deeper(self) -> DtState {
        DtState {
            stack: Stack::default(),
            definitions: self.definitions.clone(),
            context: Context::Quotation {
                parent_state: Box::new(self),
            },
        }
    }

    pub fn higher(self) -> DtState {
        let parent_state = match self.context.clone() {
            Context::Quotation { parent_state } => *parent_state,
            Context::Main => panic!("Can't escape main"),
            Context::None => panic!("Can't escape"),
        };

        parent_state.push_quote(self)
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn reverse(self) -> Self {
        self.update_stack(|stack| stack.reverse())
    }

    pub fn push(self, term: DtValue) -> Self {
        self.update_stack(|stack| stack.push(term.clone()))
    }

    pub fn push_bool(self, b: bool) -> Self {
        self.push(DtValue::Boolean(b))
    }

    pub fn push_i64(self, i: i64) -> Self {
        self.push(DtValue::I64(i))
    }

    pub fn push_f64(self, n: f64) -> Self {
        self.push(DtValue::F64(n))
    }

    pub fn push_command(self, op_name: &str) -> Self {
        self.push(DtValue::Command(op_name.to_owned()))
    }

    pub fn push_quote(self, quote: DtState) -> Self {
        self.push(DtValue::Quote(quote))
    }

    pub fn push_stab(self, st: Stab) -> Self {
        self.push(DtValue::Stab(st))
    }

    pub fn push_string(self, s: String) -> Self {
        self.push(DtValue::String(s))
    }

    pub fn push_str(self, s: &str) -> Self {
        self.push(DtValue::String(s.to_owned()))
    }

    pub fn pop(self) -> (DtValue, Self) {
        let (value, stack) = self.stack.clone().pop();
        (value, self.replace_stack(stack))
    }

    pub fn pop_bool(self, context: &str) -> (bool, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Boolean(b) => (b, quote),
            _ => panic!("{}", type_panic_msg(context, "bool", value)),
        }
    }

    pub fn pop_i64(self, context: &str) -> (i64, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::I64(n) => (n, quote),
            _ => panic!("{}", type_panic_msg(context, "i64", value)),
        }
    }

    pub fn pop_f64(self, context: &str) -> (f64, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::F64(n) => (n, quote),
            _ => panic!("{}", type_panic_msg(context, "f64", value)),
        }
    }

    fn _pop_command(self, context: &str) -> (String, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Command(op) => (op, quote),
            _ => panic!("{}", type_panic_msg(context, "command", value)),
        }
    }

    pub fn pop_quote(self, context: &str) -> (DtState, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Quote(subquote) => (subquote, quote),
            // TODO: Can we coerce somehow?
            // DtValue::Stab(s) => (stab_to_quote(s), quote),
            _ => panic!("{}", type_panic_msg(context, "quote", value)),
        }
    }

    pub fn pop_stab(self, context: &str) -> (Stab, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Stab(s) => (s, quote),
            // TODO: Can we coerce somehow?
            // DtValue::Quote(q) => (quote_to_stab(q.stack), quote),
            _ => panic!("{}", type_panic_msg(context, "string", value)),
        }
    }

    pub fn pop_stab_entry(self, context: &str) -> (String, DtValue, Self) {
        let (original_entry, quote) = self.pop_quote(context);
        let (value, entry) = original_entry.clone().stack.pop();
        let (key, entry) = entry.pop_string(context);

        if !entry.is_empty() {
            panic!(
                "{}",
                type_panic_msg(context, "[ string a ]", DtValue::Quote(original_entry))
            );
        }

        (key, value, quote)
    }

    pub fn pop_string(self, context: &str) -> (String, Self) {
        let (value, quote) = self.pop();
        match value {
            DtValue::String(s) => (s, quote),
            _ => panic!("{}", type_panic_msg(context, "string", value)),
        }
    }

    pub fn enqueue(self, value: DtValue) -> Self {
        let stack = self.stack.clone().enqueue(value);
        self.replace_stack(stack)
    }

    pub fn dequeue(self) -> (DtValue, Self) {
        let (value, stack) = self.stack.clone().dequeue();
        (value, self.replace_stack(stack))
    }
}

impl Default for DtState {
    fn default() -> Self {
        Self::new(Context::Main)
    }
}

#[derive(Clone)]
pub enum Context {
    Main,
    Quotation { parent_state: Box<DtState> },
    None,
}

pub enum DtType {
    A,
    B,
    C,
    /// Zero or many unknown types.
    Unknown,
    Boolean,
    Number,
    I64,
    F64,
    Command,
    // TODO: have quotes with typed contents
    // Examples: Quote<String...> for split
    //           Quote<String, Unknown> for stab entries
    Quote,
    QuoteOrCommand,
    QuoteOrString,
    String,
    Stab,
}

impl Display for DtType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DtType::*;
        let my_type = match self {
            A => "a",
            B => "b",
            C => "c",
            Unknown => "...",
            Boolean => "bool",
            Number => "num",
            I64 => "i64",
            F64 => "f64",
            Command => "command",
            Quote => "quote",
            QuoteOrCommand => "quote|command",
            QuoteOrString => "quote|string",
            String => "string",
            Stab => "stab",
        };

        write!(fmt, "{}", my_type)
    }
}

#[derive(Clone)]
pub enum DtValue {
    Boolean(bool),
    // TODO: Make a "Numeric" typeclass. (And floating-point/rational numbers)
    I64(i64),
    F64(f64),
    Command(String),
    Quote(DtState),
    String(String),
    Stab(Stab),
}

impl PartialEq for DtValue {
    fn eq(&self, other: &Self) -> bool {
        use DtValue::*;
        match (self, other) {
            (Boolean(a), Boolean(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (I64(a), F64(b)) => *a as f64 == *b,
            (F64(a), I64(b)) => *a == *b as f64,
            (F64(a), F64(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Command(a), Command(b)) => a == b,
            // TODO: For quotes, what about differing dictionaries? For simple lists they don't matter, for closures they do.
            (Quote(a), Quote(b)) => a.stack == b.stack,
            (Stab(a), Stab(b)) => a == b,
            _ => false,
        }
    }
}

impl DtValue {
    pub fn type_name(&self) -> String {
        self.get_type().to_string()
    }

    fn get_type(&self) -> DtType {
        match self {
            DtValue::Boolean(_) => DtType::Boolean,
            DtValue::I64(_) => DtType::I64,
            DtValue::F64(_) => DtType::F64,
            DtValue::Command(_) => DtType::Command,
            DtValue::Quote(_) => DtType::Quote,
            DtValue::String(_) => DtType::String,
            DtValue::Stab(_) => DtType::Stab,
        }
    }
}

impl std::fmt::Display for DtValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use DtValue::*;
        match self {
            Boolean(b) => write!(fmt, "{}", if *b { "true" } else { "false" }),
            I64(n) => write!(fmt, "{}", n),
            F64(n) => write!(fmt, "{}", n),
            Command(o) => write!(fmt, "{}", o),
            Quote(q) => write!(fmt, "{}", q.stack),
            String(s) => write!(fmt, "\"{}\"", s.replace('\n', "\\n")),
            Stab(t) => {
                write!(fmt, "[ ").unwrap();

                for (k, v) in t.iter() {
                    write!(fmt, "[ \"{}\" {} ] ", k, v).unwrap();
                }

                write!(fmt, "]")
            }
        }
    }
}

#[derive(Clone)]
pub struct Stack {
    pub values: Vector<DtValue>,
}

impl PartialEq for Stack {
    // FIXME: Not equal if inequal shadows (same name, diff binding) exist in the values
    fn eq(&self, other: &Self) -> bool {
        self.values
            .clone()
            .into_iter()
            .zip(other.values.clone())
            .all(|(a, b)| a == b)
    }
}

impl Stack {
    pub fn new(values: Vector<DtValue>) -> Self {
        Stack { values }
    }

    pub fn of(value: DtValue) -> Self {
        let mut values = Vector::default();
        values.push_back(value);
        Stack { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn reverse(&self) -> Stack {
        let values = self.values.iter().rev().cloned().collect();
        Stack::new(values)
    }

    pub fn push(mut self, term: DtValue) -> Stack {
        self.values.push_back(term);
        self
    }

    pub fn pop(mut self) -> (DtValue, Stack) {
        let term = self.values.pop_back().unwrap();
        (term, self)
    }

    pub fn pop_bool(self, context: &str) -> (bool, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Boolean(b) => (b, quote),
            _ => panic!("{}", type_panic_msg(context, "bool", value)),
        }
    }

    pub fn pop_i64(self, context: &str) -> (i64, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::I64(n) => (n, quote),
            _ => panic!("{}", type_panic_msg(context, "i64", value)),
        }
    }

    pub fn pop_f64(self, context: &str) -> (f64, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::F64(n) => (n, quote),
            _ => panic!("{}", type_panic_msg(context, "f64", value)),
        }
    }

    fn _pop_command(self, context: &str) -> (String, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Command(op) => (op, quote),
            _ => panic!("{}", type_panic_msg(context, "command", value)),
        }
    }

    pub fn pop_quote(self, context: &str) -> (DtState, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Quote(subquote) => (subquote, quote),
            // TODO: Can we coerce somehow?
            // DtValue::Stab(s) => (stab_to_quote(s), quote),
            _ => panic!("{}", type_panic_msg(context, "quote", value)),
        }
    }

    pub fn pop_stab(self, context: &str) -> (Stab, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::Stab(s) => (s, quote),
            // TODO: Can we coerce somehow?
            // DtValue::Quote(q) => (quote_to_stab(q.values), quote),
            _ => panic!("{}", type_panic_msg(context, "string", value)),
        }
    }

    pub fn pop_stab_entry(self, context: &str) -> (String, DtValue, Stack) {
        let (original_entry, quote) = self.pop_quote(context);
        let (value, entry) = original_entry.clone().stack.pop();
        let (key, entry) = entry.pop_string(context);

        if !entry.is_empty() {
            panic!(
                "{}",
                type_panic_msg(context, "[ string a ]", DtValue::Quote(original_entry))
            );
        }

        (key, value, quote)
    }

    pub fn pop_string(self, context: &str) -> (String, Stack) {
        let (value, quote) = self.pop();
        match value {
            DtValue::String(s) => (s, quote),
            _ => panic!("{}", type_panic_msg(context, "string", value)),
        }
    }

    pub fn enqueue(mut self, value: DtValue) -> Stack {
        self.values.push_front(value);
        self
    }

    pub fn dequeue(mut self) -> (DtValue, Stack) {
        let value = self.values.pop_front().unwrap();
        (value, self)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new(Vector::default())
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

pub type Dictionary = HashMap<String, Definition<'static>>;

pub fn dictionary_of<Entries>(entries: Entries) -> Dictionary
where
    Entries: IntoIterator<Item = (String, Definition<'static>)>,
{
    HashMap::from_iter(entries)
}

pub type Stab = HashMap<String, DtValue>;

pub fn new_stab() -> Stab {
    HashMap::new()
}

#[derive(Clone)]
pub struct Definition<'a> {
    pub name: String,
    consumes: &'a [DtType],
    produces: &'a [DtType],
    action: DtAction<'a>,
}

#[derive(Clone)]
pub enum DtAction<'a> {
    Builtin(Arc<dyn Fn(DtState) -> DtState + 'a>),
    Quotation(DtState),
}

impl Definition<'_> {
    pub fn on_state<'a, F>(
        name: &str,
        consumes: &'a [DtType],
        produces: &'a [DtType],
        state_action: F,
    ) -> Definition<'a>
    where
        F: Fn(DtState) -> DtState + 'a,
    {
        Definition {
            name: name.to_string(),
            consumes,
            produces,
            action: DtAction::Builtin(Arc::new(state_action)),
        }
    }

    pub fn on_jailed_state<'a, F>(
        name: &str,
        consumes: &'a [DtType],
        produces: &'a [DtType],
        state_action: F,
    ) -> Definition<'a>
    where
        F: Fn(DtState) -> DtState + 'a,
    {
        Definition {
            name: name.to_string(),
            consumes,
            produces,
            action: DtAction::Builtin(Arc::new(move |state| {
                let definitions = state.definitions.clone();
                let substate = state_action(state);
                substate.replace_definitions(definitions)
            })),
        }
    }

    pub fn contextless<'a, F>(
        name: &str,
        consumes: &'a [DtType],
        produces: &'a [DtType],
        contextless_action: F,
    ) -> Definition<'a>
    where
        F: Fn() + 'a,
    {
        Definition::on_state(name, consumes, produces, move |state| {
            contextless_action();
            state
        })
    }

    pub fn from_quote<'a>(name: &str, quote: DtState) -> Definition<'a> {
        // TODO: Infer quote effects
        Definition {
            name: name.to_string(),
            consumes: &[],
            produces: &[],
            action: DtAction::Quotation(quote),
        }
    }

    pub fn act(&mut self, state: DtState) -> DtState {
        if state.stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            log_warn(format!(
                "Underflow for \"{}\" (takes: {}, gives: {}). State: {}",
                self.name,
                self.display_consumes(),
                self.display_produces(),
                state.stack
            ));
            return state;
        }

        // TODO: Type checks?

        match &self.action {
            DtAction::Builtin(action) => action(state),
            DtAction::Quotation(quote) => quote.clone().run_in_state(state),
        }
    }

    fn display_consumes(&self) -> String {
        self.consumes
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn display_produces(&self) -> String {
        self.produces
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// The following are all handling for errors, warnings, and panics.
// TODO: Update places these are referenced to return Result.

pub fn exit_for_unknown_command(name: &str) -> ! {
    log_exit(format!("Unknown command '{}'", name));
    std::process::exit(1)
}

pub fn type_panic_msg(context: &str, expected: &str, actual: DtValue) -> String {
    format!(
        "[Context: {}] Wanted {}, but got {}",
        context, expected, actual
    )
}

pub fn log_warn(thing: impl Display) {
    let msg = format!("WARN: {}", thing).dimmed().red();
    eprintln!("{}", msg);
}

pub fn log_exit(thing: impl Display) {
    let msg = format!("RIP: {}", thing).dimmed().red();
    eprintln!("{}", msg);
}
