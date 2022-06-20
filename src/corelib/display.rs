use crate::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new(".", &["a"], &[], |state| {
            let mut stack = state.stack.clone();
            println!("{}", stack.pop().unwrap());
            state.update_stack(stack)
        }),
        RailOp::new(".s", &[], &[], |state| {
            println!("{}", state.stack);
            state
        }),
    ]
}
