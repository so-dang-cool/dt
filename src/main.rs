use std::sync::Arc;
use rustyline::Editor;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("rail {}", RAIL_VERSION);

    let mut editor = Editor::<()>::new();

    let mut stack = Stack::new();

    let mut dictionary = new_dictionary();

    let mut saving = false;

    loop {
        let input = editor.readline("> ");

        if let Err(e) = input {
            eprintln!("Derailed: {:?}", e);
            eprintln!("Final state:\n{}", stack);
            std::process::exit(1);
        }

        let input = input.unwrap();

        editor.add_history_entry(&input);

        let terms = input.split_whitespace().collect::<Vec<_>>();

        for term in terms {
            // TODO: not sufficient to have nested substacks yet.
            if saving {
                if let "]" = term {
                    saving = false;

                    let mut substack = Stack::new();
                    let mut next = || stack.pop().expect("Did not find \"[\" to begin a substack.");
                    let mut curr = next();
                
                    while curr != RailTerm::SubstackStart {
                        substack.pushl(curr);
                        curr = next();
                    }
                
                    stack.push(RailTerm::Substack(substack))
                } else {
                    stack.push(RailTerm::Unparsed(term.to_string()));
                }
            } else if let "[" = term {
                saving = true;
                stack.push(RailTerm::SubstackStart);
            } else {
                let from_dict = dictionary.iter().find(|op| op.name == term);
                if let Some(op) = from_dict {
                    let mut op = op.clone();
                    op.go(&mut stack, &mut dictionary);
                } else if let (Some('"'), Some('"')) = (term.chars().next(), term.chars().last()) {
                    let s = term.chars().skip(1).take(term.len() - 2).collect();
                    stack.push(RailTerm::String(s));
                } else if let Ok(i) = term.parse::<i64>() {
                    stack.push(RailTerm::I64(i));
                } else {
                    eprintln!("Derailed: unknown term {:?}", term);
                    std::process::exit(1);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RailTerm {
    I64(i64),
    String(String),
    SubstackStart,
    Unparsed(String),
    Substack(Stack),
}

impl std::fmt::Display for RailTerm {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use RailTerm::*;
        match self {
            I64(n) => write!(fmt, "{:?}", n),
            String(s) => write!(fmt, "\"{}\"", s),
            SubstackStart => write!(fmt, "["),
            Unparsed(s) => write!(fmt, "{}", s),
            Substack(s) => write!(fmt, "({:?})", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    fn pushl(&mut self, term: RailTerm) {
        self.terms.insert(0, term)
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
    op: Arc<dyn FnMut(&mut Stack) + 'static>,
}

impl RailOp<'_> {
    fn new<'a>(
        name: &'a str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: Arc<dyn FnMut(&mut Stack) + 'static>,
    ) -> RailOp<'a>
    {
        RailOp {
            name,
            consumes,
            produces,
            op,
        }
    }

    fn new_raw<'a, F>(
        name: &'a str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: F,
    ) -> RailOp<'a>
        where F: FnMut(&mut Stack) + 'static
     
    {
        RailOp {
            name,
            consumes,
            produces,
            op: Arc::new(op),
        }
    }

    fn go(&mut self, stack: &mut Stack, dictionary: &mut Dictionary) {
        if stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            eprintln!(
                "Derailed: stack underflow for \"{}\" ({} -> {}) -- wanted {} but stack only had {}",
                self.name,
                self.consumes.join(" "),
                self.produces.join(" "),
                self.consumes.len(),
                stack.len()
            );
            std::process::exit(1);
        }

        // TODO: Type checks
        let mut op = &self.op;

        op(stack);
    }
}

type Dictionary = Vec<RailOp<'static>>;

fn new_dictionary() -> Dictionary {
    vec![
        RailOp::new_raw(".", &["a"], &[], |stack| {
            println!("{:?}", stack.pop().unwrap())
        }),
        RailOp::new_raw(".s", &[], &[], |stack| println!("{}", stack)),
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
        RailOp::new_raw("drop", &["a"], &[], |stack| {
            stack.pop().unwrap();
        }),
        RailOp::new_raw("dup", &["a"], &["a", "a"], |stack| {
            let a = stack.pop().unwrap();
            stack.push(a.clone());
            stack.push(a);
        }),
        RailOp::new_raw("swap", &["b", "a"], &["a", "b"], |stack| {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a);
            stack.push(b);
        }),
        RailOp::new_raw("rot", &["c", "b", "a"], &["a", "c", "b"], |stack| {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let c = stack.pop().unwrap();
            stack.push(a);
            stack.push(c);
            stack.push(b);
        }),
    ]
}

// Operations

fn unary_op<'a, F>(op: F) -> Arc<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    Arc::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        match a {
            RailTerm::I64(a) => {
                let res = op(a);
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
    })
}

fn binary_op<'a, F>(op: F) -> Arc<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    Arc::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailTerm::I64(a), RailTerm::I64(b)) => {
                let res = op(a, b);
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
    })
}

// Predicates

fn unary_pred<'a, F>(op: F) -> Arc<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64) -> bool + Sized + 'a,
{
    Arc::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        match a {
            RailTerm::I64(a) => {
                let res = if op(a) { 1 } else { 0 };
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
    })
}

fn binary_pred<'a, F>(op: F) -> Arc<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    Arc::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailTerm::I64(a), RailTerm::I64(b)) => {
                let res = if op(a, b) { 1 } else { 0 };
                stack.push(RailTerm::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
    })
}
