use crate::dt_machine::{Definition, DtType};

use DtType::{A, B, C};

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        Definition::on_state("drop", &[A], &[], |quote| quote.pop().1),
        Definition::on_state("dup", &[A], &[A, A], |quote| {
            let (a, quote) = quote.pop();
            quote.push(a.clone()).push(a)
        }),
        Definition::on_state("dup2", &[A, B], &[A, B, A, B], |quote| {
            let (b, quote) = quote.pop();
            let (a, quote) = quote.pop();
            quote.push(a.clone()).push(b.clone()).push(a).push(b)
        }),
        Definition::on_state("swap", &[A, B], &[B, A], |quote| {
            let (a, quote) = quote.pop();
            let (b, quote) = quote.pop();
            quote.push(a).push(b)
        }),
        Definition::on_state("rot", &[A, B, C], &[C, A, B], |quote| {
            let (a, quote) = quote.pop();
            let (b, quote) = quote.pop();
            let (c, quote) = quote.pop();
            quote.push(a).push(c).push(b)
        }),
    ]
}
