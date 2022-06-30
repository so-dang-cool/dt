use crate::rail_machine::{Quote, RailDef};

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack("type", &["a"], &["string"], |stack| {
            let (thing, stack) = stack.pop();
            stack.push_string(thing.type_name())
        }),
        RailDef::on_state("dict", &[], &["quot"], |state| {
            state.update_stack_and_dict(|stack, dictionary| {
                let mut defs = dictionary.keys().collect::<Vec<_>>();
                defs.sort();
                let quot = defs
                    .iter()
                    .fold(Quote::default(), |stack, def| stack.push_str(def));
                let stack = stack.push_quote(quot);
                (stack, dictionary)
            })
        }),
    ]
}
