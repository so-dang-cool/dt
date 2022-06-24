use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_stack("type", &["a"], &["string"], |stack| {
        let (thing, stack) = stack.pop();
        stack.push_string(thing.type_name())
    })]
}
