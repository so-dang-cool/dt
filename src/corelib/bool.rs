use crate::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        push_bool("true", true),
        push_bool("false", false),
        RailOp::new("not", &["bool"], &["bool"], |state| {
            let mut stack = state.stack.clone();
            let b = stack.pop_bool("not");
            stack.push_bool(!b);
            state.update_stack(stack)
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
        let mut stack = state.stack.clone();
        stack.push_bool(b);
        state.update_stack(stack)
    })
}

enum Equality {
    Equal,
    NotEqual,
}

fn equality(name: &str, eq: Equality) -> RailOp<'_> {
    RailOp::new(name, &["a", "a"], &["bool"], move |state| {
        let mut stack = state.stack.clone();
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        use RailVal::*;

        let res = match (a, b) {
            (Boolean(a), Boolean(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (String(a), String(b)) => a == b,
            (a, b) => panic!("Cannot compare equality of {:?} and {:?}", a, b),
        };

        let res = match eq {
            Equality::Equal => res,
            Equality::NotEqual => !res,
        };

        stack.push_bool(res);

        state.update_stack(stack)
    })
}

fn binary_i64_pred<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailOp::new(name, &["i64", "i64"], &["bool"], move |state| {
        let mut stack = state.stack.clone();
        let b = stack.pop_i64(name);
        let a = stack.pop_i64(name);
        stack.push_bool(op(a, b));
        state.update_stack(stack)
    })
}
