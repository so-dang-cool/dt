use crate::rail_machine::{self, RailDef, RailVal};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        push_bool("true", true),
        push_bool("false", false),
        RailDef::on_quote("not", &["bool"], &["bool"], |quote| {
            let (b, quote) = quote.pop_bool("not");
            quote.push_bool(!b)
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
    RailDef::on_quote(name, &[], &["bool"], move |quote| quote.push_bool(b))
}

enum Equality {
    Equal,
    NotEqual,
}

fn equality(name: &str, eq: Equality) -> RailDef<'_> {
    RailDef::on_quote(name, &["a", "a"], &["bool"], move |quote| {
        let (b, quote) = quote.pop();
        let (a, quote) = quote.pop();

        use RailVal::*;

        let res = match (a, b) {
            (Boolean(a), Boolean(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (I64(a), F64(b)) => a as f64 == b,
            (F64(a), I64(b)) => a == b as f64,
            (F64(a), F64(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Command(a), Command(b)) => a == b,
            (Quote(a), Quote(b)) => a == b,
            _ => false,
        };

        let res = match eq {
            Equality::Equal => res,
            Equality::NotEqual => !res,
        };

        quote.push_bool(res)
    })
}

fn binary_numeric_pred<'a, F, G>(name: &'a str, f64_op: F, i64_op: G) -> RailDef<'a>
where
    F: Fn(f64, f64) -> bool + Sized + 'a,
    G: Fn(i64, i64) -> bool + Sized + 'a,
{
    RailDef::on_quote(name, &["num", "num"], &["bool"], move |quote| {
        let (b, quote) = quote.pop();
        let (a, quote) = quote.pop();

        use RailVal::*;
        match (a, b) {
            (I64(a), I64(b)) => quote.push_bool(i64_op(a, b)),
            (I64(a), F64(b)) => quote.push_bool(f64_op(a as f64, b)),
            (F64(a), I64(b)) => quote.push_bool(f64_op(a, b as f64)),
            (F64(a), F64(b)) => quote.push_bool(f64_op(a, b)),
            (a, b) => {
                rail_machine::log_warn(format!(
                    "Can only perform {} on numeric values but got {} and {}",
                    name, a, b
                ));
                quote.push(a).push(b)
            }
        }
    })
}
