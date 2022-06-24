use crate::rail_machine::{RailDef, RailVal};

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
        binary_i64_pred(">", |a, b| a > b),
        binary_i64_pred("<", |a, b| a < b),
        binary_i64_pred(">=", |a, b| a >= b),
        binary_i64_pred("<=", |a, b| a <= b),
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

fn binary_i64_pred<'a, F>(name: &'a str, op: F) -> RailDef<'a>
where
    F: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailDef::on_stack(name, &["i64", "i64"], &["bool"], move |stack| {
        let (b, stack) = stack.pop_i64(name);
        let (a, stack) = stack.pop_i64(name);
        stack.push_bool(op(a, b))
    })
}
