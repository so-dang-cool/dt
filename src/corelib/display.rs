use crate::rail_machine::{RailDef, RailVal};
use crate::RAIL_VERSION;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack("print", &["a"], &[], |stack| {
            let (a, stack) = stack.pop();
            match a {
                RailVal::String(a) => println!("{}", a),
                _ => println!("{}", a),
            }
            stack
        }),
        RailDef::on_state("show", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
        RailDef::contextless("clear", &[], &[], || {
            clearscreen::clear().expect("Unable to clear screen")
        }),
        RailDef::on_stack("version", &[], &["string"], |stack| stack.push_str(RAIL_VERSION)),
    ]
}
