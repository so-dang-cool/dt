use crate::Context;
use crate::RailState;
use crate::RailVal;
use std::fmt::Debug;
use std::sync::Arc;

pub fn operate(state: RailState, term: String) -> RailState {
    let mut stack = state.stack.clone();
    let mut dictionary = state.dictionary.clone();
    let context = state.context.clone();

    if term == "[" {
        return state.deeper();
    } else if term == "]" {
        return state.higher();
    } else if let Some(op) = dictionary.iter_mut().find(|op| op.name == term) {
        if let Context::Main = context {
            return op.go(state.clone());
        } else {
            stack.push(RailVal::Operator(op.clone()));
        }
    } else if let (Some('"'), Some('"')) = (term.chars().next(), term.chars().last()) {
        let s = term.chars().skip(1).take(term.len() - 2).collect();
        stack.push(RailVal::String(s));
    } else if let Ok(i) = term.parse::<i64>() {
        stack.push(RailVal::I64(i));
    } else {
        eprintln!("Derailed: unknown term {:?}", term);
        std::process::exit(1);
    }

    RailState {
        stack,
        dictionary,
        context,
    }
}

pub fn new_dictionary() -> Dictionary {
    vec![
        RailOp::new(".", &["a"], &[], |state| {
            let mut stack = state.stack.clone();
            println!("{:?}", stack.pop().unwrap());
            state.update_stack(stack)
        }),
        RailOp::new(".s", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
        unary_i64_op("abs", |a| a.abs()),
        binary_i64_op("+", |a, b| a + b),
        binary_i64_op("-", |a, b| a - b),
        binary_i64_op("*", |a, b| a * b),
        binary_i64_op("/", |a, b| a / b),
        binary_i64_op("%", |a, b| a % b),
        binary_i64_op("max", |a, b| if a >= b { a } else { b }),
        binary_i64_op("min", |a, b| if a <= b { a } else { b }),
        unary_i64_pred("!", |a| a <= 0),
        binary_i64_pred("==", |a, b| a == b),
        binary_i64_pred("!=", |a, b| a != b),
        binary_i64_pred(">", |a, b| a > b),
        binary_i64_pred("<", |a, b| a < b),
        binary_i64_pred(">=", |a, b| a >= b),
        binary_i64_pred("<=", |a, b| a <= b),
        RailOp::new("drop", &["a"], &[], |state| {
            let mut stack = state.stack.clone();
            stack.pop().unwrap();
            state.update_stack(stack)
        }),
        RailOp::new("dup", &["a"], &["a", "a"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            stack.push(a.clone());
            stack.push(a);
            state.update_stack(stack)
        }),
        RailOp::new("swap", &["b", "a"], &["a", "b"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a);
            stack.push(b);
            state.update_stack(stack)
        }),
        RailOp::new("rot", &["c", "b", "a"], &["a", "c", "b"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let c = stack.pop().unwrap();
            stack.push(a);
            stack.push(c);
            stack.push(b);
            state.update_stack(stack)
        }),
        RailOp::new("call", &["quot"], &["..."], |state| {
            let mut stack = state.stack.clone();
            let rail_val = stack.pop().unwrap();
            let state = state.update_stack(stack);
            if let RailVal::Quotation(quot) = rail_val {
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
            } else {
                panic!(
                    "call is only implemented for quotations, but got {:?}",
                    rail_val
                );
            }
        }),
    ]
}

pub type Dictionary = Vec<RailOp<'static>>;

#[derive(Clone)]
pub struct RailOp<'a> {
    name: &'a str,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    op: Arc<dyn Fn(RailState) -> RailState + 'a>,
}

impl RailOp<'_> {
    fn new<'a, F>(
        name: &'a str,
        consumes: &'a [&'a str],
        produces: &'a [&'a str],
        op: F,
    ) -> RailOp<'a>
    where
        F: Fn(RailState) -> RailState + 'a,
    {
        RailOp {
            name,
            consumes,
            produces,
            op: Arc::new(op),
        }
    }

    fn go(&mut self, state: RailState) -> RailState {
        if state.stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            eprintln!(
                "Derailed: stack underflow for \"{}\" ({:?} -> {:?}): stack only had {}",
                self.name,
                self.consumes,
                self.produces,
                state.stack.len()
            );
            std::process::exit(1);
        }

        // TODO: Type checks

        (self.op)(state)
    }
}

impl Debug for RailOp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            ": {} ({:?} -- {:?}) ... ;",
            self.name, self.consumes, self.produces
        )
    }
}

// Operations

fn unary_i64_op<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    RailOp::new(name, &["i64"], &["i64"], move |state| {
        let mut stack = state.stack.clone();
        let a = stack.pop().unwrap();
        match a {
            RailVal::I64(a) => {
                let res = op(a);
                stack.push(RailVal::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        state.update_stack(stack)
    })
}

fn binary_i64_op<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    RailOp::new(name, &["i64", "i64"], &["i64"], move |state| {
        let mut stack = state.stack.clone();
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailVal::I64(a), RailVal::I64(b)) => {
                let res = op(a, b);
                stack.push(RailVal::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        state.update_stack(stack)
    })
}

// Predicates

fn unary_i64_pred<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64) -> bool + Sized + 'a,
{
    RailOp::new(name, &["i64"], &["bool"], move |state| {
        let mut stack = state.stack.clone();
        let a = stack.pop().unwrap();
        match a {
            RailVal::I64(a) => {
                let res = if op(a) { 1 } else { 0 };
                stack.push(RailVal::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        state.update_stack(stack)
    })
}

fn binary_i64_pred<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailOp::new(name, &["i64"], &["bool"], move |state| {
        let mut stack = state.stack.clone();
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        match (a, b) {
            (RailVal::I64(a), RailVal::I64(b)) => {
                let res = if op(a, b) { 1 } else { 0 };
                stack.push(RailVal::I64(res));
            }
            _ => panic!("Attempted to do math with Strings!"),
        }
        state.update_stack(stack)
    })
}
