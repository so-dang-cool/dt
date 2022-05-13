use rustyline::Editor;

fn main() {
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

        if let Some(op) = dictionary.iter_mut().find(|op| op.name == input) {
            op.go(&mut stack);
        } else if let Ok(i) = input.parse::<i64>() {
            stack.push(i);
        } else {
            eprintln!("Derailed: unknown term {:?}", input);
        }
    }
}

type Stack = Vec<i64>;

struct RailOp {
    name: String,
    consumes: Vec<String>,
    produces: Vec<String>,
    op: Box<dyn FnMut(&mut Stack)>,
}

impl RailOp {
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

        (self.op)(stack);
    }
}

fn new_dictionary() -> Vec<RailOp> {
    vec![
        RailOp {
            name: String::from("."),
            consumes: vec![],
            produces: vec![],
            op: Box::new(|stack| println!("{:?}", stack)),
        },
        RailOp {
            name: String::from("+"),
            consumes: vec![String::from("i64"), String::from("i64")],
            produces: vec![String::from("i64")],
            op: binary_op(|a, b| a + b),
        },
        RailOp {
            name: String::from("-"),
            consumes: vec![String::from("i64"), String::from("i64")],
            produces: vec![String::from("i64")],
            op: binary_op(|a, b| a - b),
        },
        RailOp {
            name: String::from("*"),
            consumes: vec![String::from("i64"), String::from("i64")],
            produces: vec![String::from("i64")],
            op: binary_op(|a, b| a * b),
        },
        RailOp {
            name: String::from("/"),
            consumes: vec![String::from("i64"), String::from("i64")],
            produces: vec![String::from("i64")],
            op: binary_op(|a, b| a / b),
        },
        RailOp {
            name: String::from("swap"),
            consumes: vec![String::from("b"), String::from("a")],
            produces: vec![String::from("a"), String::from("b")],
            op: Box::new(|stack| {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a);
                stack.push(b);
            }),
        },
        RailOp {
            name: String::from("rot"),
            consumes: vec![String::from("c"), String::from("b"), String::from("a")],
            produces: vec![String::from("a"), String::from("c"), String::from("b")],
            op: Box::new(|stack| {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let c = stack.pop().unwrap();
                stack.push(a);
                stack.push(c);
                stack.push(b);
            }),
        },
    ]
}

fn binary_op<'a, F>(op: F) -> Box<dyn FnMut(&mut Stack) + 'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    Box::new(move |stack: &mut Stack| {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        let c = op(a, b);
        stack.push(c);
    })
}
