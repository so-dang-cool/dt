use crate::rail_machine::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("drop", &["a"], &[], |state| {
            let mut stack = state.stack.clone();
            stack.pop().unwrap();
            state.update_stack(stack)
        }),
        RailOp::new("dup", &["a"], &["a", "a"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            stack.push(a.clone());
            stack.push(a);
            state.update_stack(stack)
        }),
        RailOp::new("swap", &["b", "a"], &["a", "b"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a);
            stack.push(b);
            state.update_stack(stack)
        }),
        RailOp::new("rot", &["c", "b", "a"], &["a", "c", "b"], |state| {
            let mut stack = state.stack.clone();
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let c = stack.pop().unwrap();
            stack.push(a);
            stack.push(c);
            stack.push(b);
            state.update_stack(stack)
        }),
    ]
}
