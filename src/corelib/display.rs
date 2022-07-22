use std::fmt::Display;

use crate::rail_machine::{RailDef, RailVal};
use crate::RAIL_VERSION;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        printer("p", &|a| print!("{}", a)),
        printer("pl", &|a| println!("{}", a)),
        RailDef::contextless("nl", &[], &[], || print!("\n")),
        RailDef::on_state("status", &[], &[], |state| {
            println!("{}", state.values);
            state
        }),
        RailDef::contextless("clear", &[], &[], || {
            clearscreen::clear().expect("Unable to clear screen")
        }),
        RailDef::on_quote("version", &[], &["string"], |quote| {
            quote.push_str(RAIL_VERSION)
        }),
    ]
}

fn printer<'a, P>(name: &str, p: &'a P) -> RailDef<'a>
where
    P: Fn(&dyn Display) + 'a,
{
    RailDef::on_quote(name, &["a"], &[], move |quote| {
        let (a, quote) = quote.pop();
        match a {
            RailVal::String(a) => p(&a),
            _ => p(&a),
        }
        quote
    })
}
