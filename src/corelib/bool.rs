use crate::rail_machine::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        push_bool("true", true),
        push_bool("false", false),
        RailOp::new("not", &["bool"], &["bool"], |state| {
            state.update_stack(|stack| {
                let (b, stack) = stack.pop_bool("not");
                stack.push_bool(!b)
            })
        }),
        equality("==", Equality::Equal),
        equality("!=", Equality::NotEqual),
        binary_i64_pred(">", |a, b| a > b),
        binary_i64_pred("<", |a, b| a < b),
        binary_i64_pred(">=", |a, b| a >= b),
        binary_i64_pred("<=", |a, b| a <= b),
    ]
}

fn push_bool(name: &str, b: bool) -> RailOp<'_> {
    RailOp::new(name, &[], &["bool"], move |state| {
        state.update_stack(|stack| stack.push_bool(b))
    })
}

enum Equality {
    Equal,
    NotEqual,
}

fn equality(name: &str, eq: Equality) -> RailOp<'_> {
    RailOp::new(name, &["a", "a"], &["bool"], move |state| {
        state.update_stack(|stack| {
            let (b, stack) = stack.pop();
            let (a, stack) = stack.pop();

            use RailVal::*;

            let res = match (a, b) {
                (Boolean(a), Boolean(b)) => a == b,
                (I64(a), I64(b)) => a == b,
                (String(a), String(b)) => a == b,
                (a, b) => panic!("Cannot compare equality of {} and {}", a, b),
            };

            let res = match eq {
                Equality::Equal => res,
                Equality::NotEqual => !res,
            };

            stack.push_bool(res)
        })
    })
}

fn binary_i64_pred<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailOp::new(name, &["i64", "i64"], &["bool"], move |state| {
        state.update_stack(|stack| {
            let (b, stack) = stack.pop_i64(name);
            let (a, stack) = stack.pop_i64(name);
            stack.push_bool(op(a, b))
        })
    })
}
