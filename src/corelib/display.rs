use crate::rail_machine::{RailDef, RailVal};
use crate::RAIL_VERSION;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("print", &["a"], &[], |quote| {
            let (a, quote) = quote.pop();
            match a {
                RailVal::String(a) => print!("{}", a),
                _ => print!("{}", a),
            }
            quote
        }),
        RailDef::contextless("endl", &[], &[], || print!("\n")),
        RailDef::on_state("status", &[], &[], |state| {
            println!("{}", state.quote);
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
