use rustyline::Editor;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("rail {}", RAIL_VERSION);

    RailPrompt::new().fold(RailState::new(), operate);
}

struct RailPrompt {
    editor: Editor<()>,
    terms: Vec<String>
}

impl RailPrompt {
    fn new() -> RailPrompt {
        RailPrompt { editor: Editor::<()>::new(), terms: vec![] }
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

            self.terms = input.split_whitespace().map(|s| s.to_owned()).rev().collect::<Vec<_>>();
        }

        self.terms.pop()
     }
}

struct RailState {
    stack: Stack,
    dictionary: Dictionary,
}

impl RailState {
    fn new() -> RailState {
        RailState { stack: Stack::new(), dictionary: new_dictionary() }
    }
}

fn operate(state: RailState, term: String) -> RailState {
    let mut stack = state.stack;
    let mut dictionary = state.dictionary;

    if let Some(op) = dictionary.iter_mut().find(|op| op.name == term) {
        stack = op.go(stack);
    } else if let (Some('"'), Some('"')) = (term.chars().next(), term.chars().last()) {
        let s = term.chars().skip(1).take(term.len() - 2).collect();
        stack.push(RailTerm::String(s));
    } else if let Ok(i) = term.parse::<i64>() {
        stack.push(RailTerm::I64(i));
    } else {
        eprintln!("Derailed: unknown term {:?}", term);
        std::process::exit(1);
    }

    RailState { stack, dictionary }
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

struct RailOp<'a> {
    name: &'a str,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    op: Box<dyn Fn(Stack) -> Stack + 'a>,
}

impl RailOp<'_> {
    fn new<'a, F>(
        name: &'a str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: F,
    ) -> RailOp<'a>
    where
        F: Fn(Stack) -> Stack + 'a,
    {
        RailOp {
            name,
            consumes,
            produces,
            op: Box::new(op),
        }
    }

    fn go(&mut self, stack: Stack) -> Stack {
        if stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            eprintln!(
                "Derailed: stack underflow for \"{}\" ({:?} -> {:?}): stack only had {}",
                self.name,
                self.consumes,
                self.produces,
                stack.len()
            );
            std::process::exit(1);
        }

        // TODO: Type checks

        (self.op)(stack)
    }
}

type Dictionary = Vec<RailOp<'static>>;

fn new_dictionary() -> Dictionary {
    vec![
        RailOp::new(".", &["a"], &[], |stack| {
            let mut stack = stack;
            println!("{:?}", stack.pop().unwrap());
            stack
        }),
        RailOp::new(".s", &[], &[], |stack| {println!("{}", stack); stack }),
        RailOp::new("+", &["i64", "i64"], &["i64"], binary_op(|a, b| a + b)),
        RailOp::new("-", &["i64", "i64"], &["i64"], binary_op(|a, b| a - b)),
        RailOp::new("*", &["i64", "i64"], &["i64"], binary_op(|a, b| a * b)),
        RailOp::new("/", &["i64", "i64"], &["i64"], binary_op(|a, b| a / b)),
        RailOp::new("%", &["i64", "i64"], &["i64"], binary_op(|a, b| a % b)),
        RailOp::new("==", &["i64", "i64"], &["bool"], binary_pred(|a, b| a == b)),
        RailOp::new("!=", &["i64", "i64"], &["bool"], binary_pred(|a, b| a != b)),
        RailOp::new(">", &["i64", "i64"], &["bool"], binary_pred(|a, b| a > b)),
        RailOp::new("<", &["i64", "i64"], &["bool"], binary_pred(|a, b| a < b)),
        RailOp::new(">=", &["i64", "i64"], &["bool"], binary_pred(|a, b| a >= b)),
        RailOp::new("<=", &["i64", "i64"], &["bool"], binary_pred(|a, b| a <= b)),
        RailOp::new("!", &["bool"], &["bool"], unary_pred(|a| a <= 0)),
        RailOp::new("abs", &["i64"], &["i64"], unary_op(|a| a.abs())),
        RailOp::new(
            "max",
            &["i64", "i64"],
            &["i64"],
            binary_op(|a, b| if a >= b { a } else { b }),
        ),
        RailOp::new(
            "min",
            &["i64", "i64"],
            &["i64"],
            binary_op(|a, b| if a <= b { a } else { b }),
        ),
        RailOp::new("drop", &["a"], &[], |stack| {
            let mut stack = stack;
            stack.pop().unwrap();
            stack
        }),
        RailOp::new("dup", &["a"], &["a", "a"], |stack| {
            let mut stack = stack;
            let a = stack.pop().unwrap();
            stack.push(a.clone());
            stack.push(a);
            stack
        }),
        RailOp::new("swap", &["b", "a"], &["a", "b"], |stack| {
            let mut stack = stack;
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a);
            stack.push(b);
            stack
        }),
        RailOp::new("rot", &["c", "b", "a"], &["a", "c", "b"], |stack| {
            let mut stack = stack;
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let c = stack.pop().unwrap();
            stack.push(a);
            stack.push(c);
            stack.push(b);
            stack
        }),
    ]
}

// Operations

fn unary_op<'a, F>(op: F) -> Box<dyn Fn(Stack) -> Stack + 'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    Box::new(move |stack: Stack| {
        let mut stack = stack;
        let a = stack.pop().unwrap();
        match a {
            RailTerm::I64(a) => {
                let res = op(a);
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        stack
    })
}

fn binary_op<'a, F>(op: F) -> Box<dyn Fn(Stack) -> Stack + 'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    Box::new(move |stack: Stack| {
        let mut stack = stack;
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailTerm::I64(a), RailTerm::I64(b)) => {
                let res = op(a, b);
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        stack
    })
}

// Predicates

fn unary_pred<'a, F>(op: F) -> Box<dyn Fn(Stack) -> Stack + 'a>
where
    F: Fn(i64) -> bool + Sized + 'a,
{
    Box::new(move |stack: Stack| {
        let mut stack = stack;
        let a = stack.pop().unwrap();
        match a {
            RailTerm::I64(a) => {
                let res = if op(a) { 1 } else { 0 };
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        stack
    })
}

fn binary_pred<'a, F>(op: F) -> Box<dyn Fn(Stack) -> Stack + 'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    Box::new(move |stack: Stack| {
        let mut stack = stack;
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailTerm::I64(a), RailTerm::I64(b)) => {
                let res = if op(a, b) { 1 } else { 0 };
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        stack
    })
}
