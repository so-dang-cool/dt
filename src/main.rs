use rustyline::Editor;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("rail {}", RAIL_VERSION);

    let mut editor = Editor::<()>::new();

    let mut stack: Vec<i64> = vec![];

    let mut dictionary = new_dictionary();

    loop {
        let input = editor.readline("> ");

        if let Err(e) = input {
            eprintln!("Final state: {:?}", stack);
            eprintln!("Derailed: {:?}", e);
            std::process::exit(1);
        }

        let input = input.unwrap();

        editor.add_history_entry(&input);

        let terms = input.split_whitespace();

        for term in terms {
            if let Some(op) = dictionary.iter_mut().find(|op| op.name == term) {
                op.go(&mut stack);
            } else if let Ok(i) = term.parse::<i64>() {
                stack.push(i);
            } else {
                eprintln!("Derailed: unknown term {:?}", term);
                std::process::exit(1);
            }
        }
    }
}

type Stack = Vec<i64>;

struct RailOp<'a> {
    name: &'a str,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    op: Box<dyn FnMut(&mut Stack) + 'a>,
}

impl RailOp<'_> {
    fn new<'a, F>(
        name: &'a str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: F,
    ) -> RailOp<'a>
    where
        F: FnMut(&mut Stack) + 'a,
    {
        RailOp {
            name,
            consumes,
            produces,
            op: Box::new(op),
        }
    }

    fn go(&mut self, stack: &mut Stack) {
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

        (self.op)(stack);
    }
}

fn new_dictionary() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new(".", &["a"], &[], |stack| {
            println!("{:?}", stack.pop().unwrap())
        }),
        RailOp::new(".s", &[], &[], |stack| println!("{:?}", stack)),
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
            stack.pop().unwrap();
        }),
        RailOp::new("dup", &["a"], &["a", "a"], |stack| {
            let a = stack.pop().unwrap();
            stack.push(a);
            stack.push(a);
        }),
        RailOp::new("swap", &["b", "a"], &["a", "b"], |stack| {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a);
            stack.push(b);
        }),
        RailOp::new("rot", &["c", "b", "a"], &["a", "c", "b"], |stack| {
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

fn unary_op<'a, F>(op: F) -> Box<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    Box::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let res = op(a);
        stack.push(res);
    })
}

fn binary_op<'a, F>(op: F) -> Box<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    Box::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        let res = op(a, b);
        stack.push(res);
    })
}

// Predicates

fn unary_pred<'a, F>(op: F) -> Box<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64) -> bool + Sized + 'a,
{
    Box::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let res = if op(a) { 1 } else { 0 };
        stack.push(res);
    })
}

fn binary_pred<'a, F>(op: F) -> Box<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    Box::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        let res = if op(a, b) { 1 } else { 0 };
        stack.push(res);
    })
}
