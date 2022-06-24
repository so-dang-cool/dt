use crate::rail_machine::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::on_stack(".", &["a"], &[], |stack| {
            let (a, stack) = stack.pop();
            println!("{}", a);
            stack
        }),
        RailOp::on_state(".s", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
    ]
}
