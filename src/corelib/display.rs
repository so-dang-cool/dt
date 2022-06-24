use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack(".", &["a"], &[], |stack| {
            let (a, stack) = stack.pop();
            println!("{}", a);
            stack
        }),
        RailDef::on_stack("print", &["string"], &[], |stack| {
            let (a, stack) = stack.pop_string("print");
            println!("{}", a);
            stack
        }),
        RailDef::on_state(".s", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
    ]
}
