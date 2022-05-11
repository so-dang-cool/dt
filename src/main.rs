use rustyline::Editor;

fn main() {
    let mut editor = Editor::<()>::new();

    let mut stack: Vec<i64> = vec![];

    loop {
        let input = editor.readline("> ");

        if let Err(e) = input {
            eprintln!("Derailed: {:?}", e);
            std::process::exit(1);
        }

        let input = input.unwrap();

        editor.add_history_entry(&input);

        let term = RailTerm::from_str(&input);

        match term {
            RailTerm::Dot => println!("{:?}", stack),
            RailTerm::Int(i) => stack.push(i),
            RailTerm::MathOp(op) => {
                if stack.len() < 2 {
                    eprintln!("Derailed: stack underflow {:?}", stack);
                    std::process::exit(1);
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(match op {
                    RailMathOp::Plus => a + b,
                    RailMathOp::Minus => a - b,
                    RailMathOp::Times => a * b,
                    RailMathOp::DivBy => a / b,
                })
            }
            RailTerm::Unknown(term) => {
                eprintln!("Derailed: unknown term {:?}", term);
                std::process::exit(1);
            }
        }
    }
}

enum RailTerm {
    Dot,
    Int(i64),
    MathOp(RailMathOp),
    Unknown(String),
}

enum RailMathOp {
    Plus,
    Minus,
    Times,
    DivBy,
}

impl RailTerm {
    // TODO: Convert to actual FromStr impl.
    fn from_str(s: &str) -> Self {
        use RailMathOp::*;
        use RailTerm::*;
        match s {
            "." => Dot,
            "+" => MathOp(Plus),
            "-" => MathOp(Minus),
            "*" => MathOp(Times),
            "/" => MathOp(DivBy),
            _ => match s.parse::<i64>() {
                Ok(i) => Int(i),
                Err(_) => Unknown(s.to_string()),
            },
        }
    }
}
