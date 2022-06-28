use crate::rail_machine::{type_panic_msg, RailDef, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        push_bool("true", true),
        push_bool("false", false),
        RailDef::on_stack("not", &["bool"], &["bool"], |stack| {
            let (b, stack) = stack.pop_bool("not");
            stack.push_bool(!b)
        }),
        equality("==", Equality::Equal),
        equality("!=", Equality::NotEqual),
        binary_numeric_pred(">", |a, b| a > b, |a, b| a > b),
        binary_numeric_pred("<", |a, b| a < b, |a, b| a < b),
        binary_numeric_pred(">=", |a, b| a >= b, |a, b| a >= b),
        binary_numeric_pred("<=", |a, b| a <= b, |a, b| a <= b),
    ]
}

fn push_bool(name: &str, b: bool) -> RailDef<'_> {
    RailDef::on_stack(name, &[], &["bool"], move |stack| stack.push_bool(b))
}

enum Equality {
    Equal,
    NotEqual,
}

fn equality(name: &str, eq: Equality) -> RailDef<'_> {
    RailDef::on_stack(name, &["a", "a"], &["bool"], move |stack| {
        let (b, stack) = stack.pop();
        let (a, stack) = stack.pop();

        use RailVal::*;

        let res = match (a, b) {
            (Boolean(a), Boolean(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (I64(a), F64(b)) => a as f64 == b,
            (F64(a), I64(b)) => a == b as f64,
            (F64(a), F64(b)) => a == b,
            (String(a), String(b)) => a == b,
            (a, b) => panic!("Cannot compare equality of {} and {}", a, b),
        };

        let res = match eq {
            Equality::Equal => res,
            Equality::NotEqual => !res,
        };

        stack.push_bool(res)
    })
}

fn binary_numeric_pred<'a, F, G>(name: &'a str, f64_op: F, i64_op: G) -> RailDef<'a>
where
    F: Fn(f64, f64) -> bool + Sized + 'a,
    G: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailDef::on_stack(name, &["num", "num"], &["bool"], move |stack| {
        let (b, stack) = stack.pop();
        let (a, stack) = stack.pop();

        use RailVal::*;
        match (a, b) {
            (I64(a), I64(b)) => stack.push_bool(i64_op(a, b)),
            (I64(a), F64(b)) => stack.push_bool(f64_op(a as f64, b)),
            (F64(a), I64(b)) => stack.push_bool(f64_op(a, b as f64)),
            (F64(a), F64(b)) => stack.push_bool(f64_op(a, b)),
            (a, I64(_b)) => panic!("{}", type_panic_msg(name, "num", a)),
            (a, F64(_b)) => panic!("{}", type_panic_msg(name, "num", a)),
            (_a, b) => panic!("{}", type_panic_msg(name, "num", b)),
        }
    })
}
