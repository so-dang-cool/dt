use std::fmt::Display;

use crate::dt_machine::{Definition, DtType, DtValue};
use crate::DT_VERSION;

pub fn builtins() -> Vec<Definition<'static>> {
    vec![
        printer("p", &|a| print!("{}", a)),
        printer("pl", &|a| println!("{}", a)),
        Definition::contextless("nl", &[], &[], || print!("\n")),
        Definition::on_state("status", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
        Definition::contextless("clear", &[], &[], || {
            clearscreen::clear().expect("Unable to clear screen")
        }),
        Definition::on_state("version", &[], &[DtType::String], |quote| {
            quote.push_str(DT_VERSION)
        }),
    ]
}

fn printer<'a, P>(name: &str, p: &'a P) -> Definition<'a>
where
    P: Fn(&dyn Display) + 'a,
{
    Definition::on_state(name, &[DtType::A], &[], move |quote| {
        let (a, quote) = quote.pop();
        match a {
            DtValue::String(a) => p(&a),
            _ => p(&a),
        }
        quote
    })
}
