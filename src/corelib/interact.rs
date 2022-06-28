use crate::rail_machine::RailDef;
use clearscreen;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::contextless("clear", &[], &[], || {
        clearscreen::clear().expect("Unable to clear screen")
    })]
}
