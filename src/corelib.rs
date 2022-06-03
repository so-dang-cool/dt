use crate::RailState;
use crate::RailTerm;
use crate::Stack;

pub fn operate(state: RailState, term: String) -> RailState {
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

pub fn new_dictionary() -> Dictionary {
    vec![
        RailOp::new(".", &["a"], &[], |stack| {
            let mut stack = stack;
            println!("{:?}", stack.pop().unwrap());
            stack
        }),
        RailOp::new(".s", &[], &[], |stack| {
            println!("{}", stack);
            stack
        }),
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

pub type Dictionary = Vec<RailOp<'static>>;

pub struct RailOp<'a> {
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
