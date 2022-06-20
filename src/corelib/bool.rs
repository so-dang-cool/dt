use crate::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        unary_i64_pred("!", |a| a <= 0),
        binary_i64_pred("==", |a, b| a == b),
        binary_i64_pred("!=", |a, b| a != b),
        binary_i64_pred(">", |a, b| a > b),
        binary_i64_pred("<", |a, b| a < b),
        binary_i64_pred(">=", |a, b| a >= b),
        binary_i64_pred("<=", |a, b| a <= b),
    ]
}

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
