use crate::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        unary_i64_op("abs", |a| a.abs()),
        binary_i64_op("+", |a, b| a + b),
        binary_i64_op("-", |a, b| a - b),
        binary_i64_op("*", |a, b| a * b),
        binary_i64_op("/", |a, b| a / b),
        binary_i64_op("%", |a, b| a % b),
        binary_i64_op("max", |a, b| if a >= b { a } else { b }),
        binary_i64_op("min", |a, b| if a <= b { a } else { b }),
    ]
}

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
