use crate::rail_machine::{self, RailDef, RailType, RailVal};

use RailType::*;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        unary_numeric_op("abs", |a| a.abs(), |a| a.abs()),
        unary_numeric_op("negate", |a| -a, |a| -a),
        unary_to_f64_op("sqrt", |a| a.sqrt()),
        unary_to_i64_op("floor", |a| a),
        binary_numeric_op("+", |a, b| a + b, |a, b| a + b),
        binary_numeric_op("-", |a, b| a - b, |a, b| a - b),
        binary_numeric_op("*", |a, b| a * b, |a, b| a * b),
        binary_numeric_op("/", |a, b| a / b, |a, b| a / b),
        binary_numeric_op("mod", |a, b| a % b, |a, b| a % b),
        RailDef::on_state("int-max", &[], &[I64], |quote| quote.push_i64(i64::MAX)),
        RailDef::on_state("int-min", &[], &[I64], |quote| quote.push_i64(i64::MIN)),
        RailDef::on_state("float-max", &[], &[F64], |quote| quote.push_f64(f64::MAX)),
        RailDef::on_state("float-min", &[], &[F64], |quote| quote.push_f64(f64::MIN)),
        RailDef::on_state("digits", &[I64], &[Quote], |quote| {
            let (n, quote) = quote.pop_i64("digits");
            let ns = quote.child();
            if n == 0 {
                return quote.push_quote(ns.push_i64(0));
            }
            let mut ns = ns;
            let mut n = n;
            while n != 0 {
                ns = ns.push_i64(n % 10);
                n = n / 10;
            }
            return quote.push_quote(ns.reverse())
        })
    ]
}

fn unary_numeric_op<'a, F, G>(name: &'a str, f64_op: F, i64_op: G) -> RailDef<'a>
where
    F: Fn(f64) -> f64 + Sized + 'a,
    G: Fn(i64) -> i64 + Sized + 'a,
{
    RailDef::on_state(name, &[Number], &[Number], move |quote| {
        let (n, quote) = quote.pop();
        match n {
            RailVal::I64(n) => quote.push_i64(i64_op(n)),
            RailVal::F64(n) => quote.push_f64(f64_op(n)),
            _ => {
                rail_machine::log_warn(format!(
                    "Can only perform {} on numeric values, but got {}",
                    name, n
                ));
                quote.push(n)
            }
        }
    })
}

fn unary_to_f64_op<'a, F>(name: &'a str, f64_op: F) -> RailDef<'a>
where
    F: Fn(f64) -> f64 + Sized + 'a,
{
    RailDef::on_state(name, &[Number], &[F64], move |quote| {
        let (n, quote) = quote.pop();
        match n {
            RailVal::I64(n) => quote.push_f64(f64_op(n as f64)),
            RailVal::F64(n) => quote.push_f64(f64_op(n)),
            _ => {
                rail_machine::log_warn(format!(
                    "Can only perform {} on numeric values, but got {}",
                    name, n
                ));
                quote.push(n)
            }
        }
    })
}

fn unary_to_i64_op<'a, F>(name: &'a str, i64_op: F) -> RailDef<'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    RailDef::on_state(name, &[Number], &[I64], move |quote| {
        let (n, quote) = quote.pop();
        match n {
            RailVal::I64(n) => quote.push_i64(i64_op(n)),
            RailVal::F64(n) => quote.push_i64(i64_op(n as i64)),
            _ => {
                rail_machine::log_warn(format!(
                    "Can only perform {} on numeric values, but got {}",
                    name, n
                ));
                quote.push(n)
            }
        }
    })
}

fn binary_numeric_op<'a, F, G>(name: &'a str, f64_op: F, i64_op: G) -> RailDef<'a>
where
    F: Fn(f64, f64) -> f64 + Sized + 'a,
    G: Fn(i64, i64) -> i64 + Sized + 'a,
{
    RailDef::on_state(name, &[Number, Number], &[Number], move |quote| {
        let (b, quote) = quote.pop();
        let (a, quote) = quote.pop();

        use RailVal::*;
        match (a, b) {
            (I64(a), I64(b)) => quote.push_i64(i64_op(a, b)),
            (I64(a), F64(b)) => quote.push_f64(f64_op(a as f64, b)),
            (F64(a), I64(b)) => quote.push_f64(f64_op(a, b as f64)),
            (F64(a), F64(b)) => quote.push_f64(f64_op(a, b)),
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
