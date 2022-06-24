use crate::rail_machine::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new(".", &["a"], &[], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                println!("{}", a);
                stack
            })
        }),
        RailOp::new(".s", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
    ]
}
